use crate::api::github::{GithubRepo, GithubStatsResponse};
use crate::api::{github, wakatime};
use crate::templates;

use askama::Template;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
};
use lazy_static::lazy_static;
use moka::future::Cache;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

lazy_static! {
    static ref LANG_TO_COLORS: HashMap<String, String> =
        serde_json::from_str(include_str!("../../data/lang2hex.json")).unwrap();
}

const MAX_BAR_WIDTH: f32 = 275.0;
const DEFAULT_LANG_COLOR: &str = "#818181";

#[derive(Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Params {
    username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageStat {
    name: String,
    color: String,
    percent: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MinimalLanguageStat {
    name: String,
    count: f32,
}

#[derive(Template)]
#[template(path = "compact/languages.html")]
pub struct CompactLanguagesTemplate {
    name: String,
    stats_bar: String,
    bar_legend: String,
}

async fn get_top_langs_by_waka_intl(
    cache: Cache<String, String>,
    username: &String,
) -> Result<Vec<LanguageStat>, String> {
    let cache_key = format!("wakatime:langs:{username}");
    if let Some(cached) = cache.get(&cache_key).await {
        let langs = serde_json::from_str(&cached).unwrap();
        return Ok(langs);
    }

    let stats = wakatime::get_stats(username).await;
    if !stats.is_ok() {
        return Err("FailedFindUser".to_string());
    }

    let languages = stats.unwrap().data.languages;
    let first_languages = match languages.first_chunk::<6>() {
        Some(langs) => langs,
        None => {
            return Err("FailedFindLanguages".to_string());
        }
    };

    let max_percent = first_languages
        .iter()
        .fold(0.0, |acc, val| acc + val.percent);
    let top_langs: Vec<LanguageStat> = first_languages
        .iter()
        .map(|lang| LanguageStat {
            name: lang.name.clone(),
            color: match LANG_TO_COLORS.get(&lang.name.to_lowercase()) {
                None => DEFAULT_LANG_COLOR.to_string(),
                Some(color) => color.clone(),
            },
            percent: 100.0 / (max_percent / lang.percent),
        })
        .collect();

    let cache_body = serde_json::to_string(&top_langs).unwrap();
    cache.insert(cache_key, cache_body).await;

    Ok(top_langs)
}

async fn get_top_langs_by_github_intl(
    cache: Cache<String, String>,
    username: &String,
) -> Result<Vec<LanguageStat>, String> {
    let cache_key = format!("github:langs:{username}");
    if let Some(cached) = cache.get(&cache_key).await {
        let langs = serde_json::from_str(&cached).unwrap();
        return Ok(langs);
    }

    let stats = github::get_stats(username).await;
    if !stats.is_ok() {
        return Err("FailedFindUser".to_string());
    }

    let languages_raw_data = match stats.unwrap() {
        GithubStatsResponse::Failed(err) => {
            let err_message = if err.message.contains("Bad credentials") {
                "BadCredentials"
            } else {
                "FailedFindUser"
            };
            return Err(err_message.to_string());
        }
        GithubStatsResponse::Valid(res) => res,
    };

    let languages_data: Vec<&GithubRepo> = languages_raw_data
        .iter()
        .filter(|&lang| !lang.language.is_none())
        .collect();
    if languages_data.len() == 0 {
        return Err("FailedFindLanguages".to_string());
    }

    let mut langs_data: HashMap<String, i32> = HashMap::new();
    for lang in languages_data {
        let lang_name = lang.language.as_ref().unwrap();
        langs_data
            .entry(lang_name.to_string())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut first_languages = Vec::new();
    for _ in 0..6 {
        let possible_data = langs_data.iter().max_by(|a, b| a.1.cmp(&b.1));
        if possible_data.is_none() {
            break;
        }

        let top_lang_data = possible_data.unwrap();
        let lang_name = top_lang_data.0.to_string();
        let min_lang = MinimalLanguageStat {
            name: lang_name.to_string(),
            count: top_lang_data.1.to_owned() as f32,
        };

        first_languages.push(min_lang);
        langs_data.remove(&lang_name);
    }

    let max_percent = first_languages.iter().fold(0.0, |acc, val| acc + val.count);
    let top_langs: Vec<LanguageStat> = first_languages
        .iter()
        .map(|lang| LanguageStat {
            name: lang.name.clone(),
            color: match LANG_TO_COLORS.get(&lang.name.to_lowercase()) {
                None => DEFAULT_LANG_COLOR.to_string(),
                Some(color) => color.clone(),
            },
            percent: 100.0 / (max_percent / lang.count),
        })
        .collect();

    let cache_body = serde_json::to_string(&top_langs).unwrap();
    cache.insert(cache_key, cache_body).await;

    Ok(top_langs)
}

pub fn render_top_langs(
    username: String,
    top_langs_res: Result<Vec<LanguageStat>, String>,
) -> Response {
    if !top_langs_res.is_ok() {
        let message = top_langs_res.unwrap_err();
        let template = match message.as_str() {
            "FailedFindUser" => templates::ErrorTemplate {
                first_line: "Failed to find a user.".to_string(),
                second_line: "Check if it’s spelled correctly".to_string(),
            },
            "FailedFindLanguages" => templates::ErrorTemplate {
                first_line: "Failed to find a user languages.".to_string(),
                second_line: "Maybe he's inactive".to_string(),
            },
            "BadCredentials" => templates::ErrorTemplate {
                first_line: "Bad credentials.".to_string(),
                second_line: "Problems with service API token".to_string(),
            },
            _ => templates::ErrorTemplate {
                first_line: "Unknown API error.".to_string(),
                second_line: "Let us know about it".to_string(),
            },
        };

        let svg_template = templates::SVGTemplate(template);
        return templates::SVGTemplate::<templates::ErrorTemplate>::into_response(svg_template);
    }

    let stats = top_langs_res.unwrap();
    let mut bar_start_x = 20.0;
    let mut column_start_y = 93;

    let bar_data: Vec<String> = stats
        .iter()
        .map(|stat| {
            let stat_percent = stat.percent / 100.0;
            let block_width = MAX_BAR_WIDTH * stat_percent;
            let element = format!(
                r##"<rect mask="url(#stats_mask)" x="{bar_start_x:.2}" y="61" width="{block_width:.2}" height="10" fill="{0}" />"##,
                stat.color
            );

            bar_start_x += block_width;

            element
        })
        .collect();

    let bar_legend: Vec<String> = stats
        .iter()
        .enumerate()
        .map(|(idx, stat)| {
            let start_x = if idx < 3 { 20 } else { 175 };
            let text_x = start_x + 18;
            let start_y = column_start_y;
            let text_y = start_y + 11;
            let element = format!(
                r##"
                <g>
                    <rect x="{start_x}" y="{start_y}" width="12" height="12" rx="6" fill="{0}" />
                    <text x="{text_x}" y="{text_y}" fill="#CAD3F5" class="stat-text">{1} {2:.2}%</text>
                </g>
            "##,
                stat.color, stat.name, stat.percent,
            );

            column_start_y = if idx == 2 { 93 } else { column_start_y + 24 };

            element
        })
        .collect();

    let template = CompactLanguagesTemplate {
        name: username,
        stats_bar: bar_data.join("\n"),
        bar_legend: bar_legend.join("\n"),
    };
    let svg_template = templates::SVGTemplate(template);
    templates::SVGTemplate::<CompactLanguagesTemplate>::into_response(svg_template)
}

pub async fn get_waka_top_langs(
    State(cache): State<Cache<String, String>>,
    Query(params): Query<Params>,
) -> Response {
    let username = params.username;
    let top_langs_res = get_top_langs_by_waka_intl(cache, &username).await;
    render_top_langs(username, top_langs_res)
}

pub async fn get_github_top_langs(
    State(cache): State<Cache<String, String>>,
    Query(params): Query<Params>,
) -> Response {
    let username = params.username;
    let top_langs_res = get_top_langs_by_github_intl(cache, &username).await;
    render_top_langs(username, top_langs_res)
}
