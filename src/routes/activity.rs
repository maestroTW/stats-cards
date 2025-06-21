use crate::api::github::{
    self, ActivityResponse as GithubActivityResponse, ContributionMonth as GithubContributionMonth,
};
use crate::data::config::CONFIG;
use crate::data::theme::{ActivityColor, Theme, ThemeData};
use crate::prepared_templates::{PreparedTemplate, gh_handle_error_template};
use crate::templates;

use askama::Template;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
};
use chrono::{Duration, Utc};
use moka::future::Cache;
use serde::{Deserialize, Serialize};

const DAY_BLOCK_SIZE: i32 = 16;
const DEFAULT_START_X: i32 = 50;

#[derive(Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Params {
    username: String,
    theme: Option<Theme>,
    period: Option<String>,
    with_title: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ActivityDay {
    count: i32,
    weekday: i8,
    color: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ActivityWeek {
    days: Vec<ActivityDay>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ActivityMonth {
    name: String,
    weeks: Vec<ActivityWeek>,
}

#[derive(Template)]
#[template(path = "compact/activity.html")]
pub struct CompactActivityTemplate {
    name: String,
    theme_data: ThemeData,
    stats_data: String,
    months_legend: String,
    week_legend: String,
    width: u32,
    height: u32,
    with_title: bool,
}

pub enum Period {
    Year = 365,
    SixMonths = 180,
    ThreeMonths = 90,
}

impl Period {
    pub fn from_key(key: &str) -> Option<u32> {
        match key {
            "year" => Some(Period::Year as u32),
            "6_months" => Some(Period::SixMonths as u32),
            "3_months" => Some(Period::ThreeMonths as u32),
            _ => None,
        }
    }
}

async fn get_activity_github_intl(
    cache: Cache<String, String>,
    username: &String,
    period: &String,
) -> Result<Vec<ActivityMonth>, PreparedTemplate> {
    if username.is_empty() {
        return Err(PreparedTemplate::FailedFindUser);
    }

    let cache_key = format!("github:activity:{username}:{period}");
    if let Some(cached) = cache.get(&cache_key).await {
        let langs = serde_json::from_str(&cached).unwrap();
        return Ok(langs);
    }

    let offset_count = match Period::from_key(period.as_str()) {
        Some(offset) => offset,
        None => Period::SixMonths as u32,
    };
    let offset: chrono::TimeDelta = Duration::days(offset_count as i64);
    let end_date = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let start_date = (Utc::now() - offset).to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    let stats = github::get_activity(username, &start_date, &end_date).await;
    if !stats.is_ok() {
        return Err(PreparedTemplate::Unknown);
    }

    let user = match stats.unwrap() {
        GithubActivityResponse::Failed(err) => return Err(gh_handle_error_template(err)),
        GithubActivityResponse::Valid(res) => match res.data.user {
            None => return Err(PreparedTemplate::FailedFindUser),
            Some(user_data) => user_data,
        },
    };

    let calendar_data = user.contributions_collection.contribution_calendar;
    let weeks = calendar_data.weeks;
    let mut activity: Vec<ActivityMonth> = Vec::new();
    let mut month: Option<ActivityMonth> = None;
    for week in weeks {
        let mut week_data: Vec<ActivityDay> = Vec::new();
        for mut day in week.contribution_days {
            let _ = day.date.split_off(7);
            let finded_month_raw: Option<&GithubContributionMonth> = calendar_data
                .months
                .iter()
                .find(|month| month.first_day.contains(&day.date));
            if finded_month_raw.is_none() {
                continue;
            }

            let finded_month = finded_month_raw.unwrap();
            if month.is_none() {
                month = Some(ActivityMonth {
                    name: finded_month.name.clone(),
                    weeks: Vec::new(),
                });
            }

            if let Some(month_data) = month.as_mut() {
                if month_data.name != finded_month.name {
                    month_data.weeks.push(ActivityWeek {
                        days: week_data.clone(),
                    });
                    activity.push(month_data.clone());
                    week_data.clear();
                    month = Some(ActivityMonth {
                        name: finded_month.name.clone(),
                        weeks: Vec::new(),
                    });
                }

                let day = ActivityDay {
                    count: day.contribution_count,
                    weekday: day.weekday,
                    color: day.color,
                };

                week_data.push(day);
            }
        }

        if let Some(month_data) = month.as_mut() {
            month_data.weeks.push(ActivityWeek { days: week_data });
        }
    }

    if let Some(month_data) = month.as_ref() {
        activity.push(month_data.clone());
    }

    let cache_body = serde_json::to_string(&activity).unwrap();
    cache.insert(cache_key, cache_body).await;

    Ok(activity)
}

pub fn render_activity(
    username: String,
    with_title: bool,
    theme: Theme,
    activity_res: Result<Vec<ActivityMonth>, PreparedTemplate>,
) -> Response {
    if !activity_res.is_ok() {
        return activity_res.unwrap_err().render();
    }

    let theme_data = theme.get_data();
    let stats = activity_res.unwrap();
    let first_day = stats
        .get(0)
        .unwrap()
        .weeks
        .get(0)
        .unwrap()
        .days
        .get(0)
        .unwrap();

    let block_default_y = if with_title { 67 } else { 35 };
    let height = if with_title { 195 } else { 163 };
    let month_legend_y = block_default_y - 6;

    let mut week_legend_y = block_default_y + 28;
    let mut day_start_x = DEFAULT_START_X;
    let mut last_day_x = day_start_x;
    let mut day_start_y = block_default_y + (DAY_BLOCK_SIZE * (first_day.weekday as i32));
    let mut months_start_x = DEFAULT_START_X;
    let mut months_legend: Vec<String> = Vec::new();
    let mut month_has_one_week = false;

    let stats_data: Vec<String> = stats.iter().map(|stat| {
        let week_els: Vec<String> = stat.weeks.iter().map(|week| {
            let day_els: Vec<String> = week.days.iter().map(|day| {
                last_day_x = day_start_x;
                let day_color = match ActivityColor::from_key(&day.color.as_str()) {
                    Some(color) => theme.get_activity_color(color),
                    None => day.color.clone()
                };
                let el = format!(r##"<rect x="{day_start_x}" y="{day_start_y}" width="12" height="12" rx="2" fill="{day_color}" />"##);
                day_start_y += DAY_BLOCK_SIZE;
                if day.weekday == 6 {
                    day_start_x += DAY_BLOCK_SIZE;
                    day_start_y = block_default_y;
                }

                el
            }).collect();

            format!(r##"<g>{0}</g>"##, day_els.join("\n"))
        }).collect();

        let month_el_offset = if month_has_one_week {
            months_start_x + 8
        } else {
            months_start_x
        };
        let month_title = format!(r##"<text x="{month_el_offset}" y="{month_legend_y}" fill="{0}" class="legend-text">{1}</text>"##, theme_data.text, stat.name);
        months_legend.push(month_title);

        let weeks_count = stat.weeks.len() as i32;
        month_has_one_week = weeks_count == 1;
        let month_offset = DAY_BLOCK_SIZE * std::cmp::min(weeks_count, 4 );
        months_start_x += month_offset;
        if weeks_count >= 5 {
            months_start_x += 5;
        }

        format!(r##"<g>{0}</g>"##, week_els.join("\n"))
    }).collect();

    let width = (last_day_x + DAY_BLOCK_SIZE * 2) as u32;
    let week_legend: Vec<String> = ["Mon", "Wed", "Fri"].iter().map(|name| {
        let el = format!(r##"<text x="20" y="{week_legend_y}" fill="{0}" class="legend-text">{name}</text>"##, theme_data.text);
        week_legend_y += 32;
        el
    }).collect();

    let template = CompactActivityTemplate {
        name: username,
        theme_data,
        stats_data: stats_data.join("\n"),
        months_legend: months_legend.join("\n"),
        week_legend: week_legend.join("\n"),
        width,
        height,
        with_title,
    };
    let svg_template = templates::SVGTemplate(template);
    templates::SVGTemplate::<CompactActivityTemplate>::into_response(svg_template)
}

pub async fn get_github_activity_graph(
    State(cache): State<Cache<String, String>>,
    Query(params): Query<Params>,
) -> Response {
    let username = params.username;
    let theme = params.theme.unwrap_or(CONFIG.default_theme.clone());
    let period = match params.period {
        Some(period) => period,
        None => "3_months".to_string(),
    };
    let with_title = match params.with_title {
        Some(with_title) => with_title,
        None => true,
    };
    let activity_res = get_activity_github_intl(cache, &username, &period).await;
    render_activity(username, with_title, theme, activity_res)
}
