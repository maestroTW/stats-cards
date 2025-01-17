use crate::api;
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

#[derive(Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Params {
    username: String,
}

async fn get_top_langs_by_waka_intl(
    waka_cache: Cache<String, String>,
    username: &String,
) -> Result<Vec<LanguageStat>, String> {
    let cache_key = format!("wakatime:langs:{username}");
    if let Some(cached) = waka_cache.get(&cache_key).await {
        let langs = serde_json::from_str(&cached).unwrap();
        return Ok(langs);
    }

    let waka_stats = api::wakatime::get_stats(username).await;
    if !waka_stats.is_ok() {
        return Err("FailedGetStats".to_string());
    }

    let languages = waka_stats.unwrap().data.languages;
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
                None => "#818181".to_string(),
                Some(color) => color.clone(),
            },
            percent: 100.0 / (max_percent / lang.percent),
        })
        .collect();

    let cache_body = serde_json::to_string(&top_langs).unwrap();
    waka_cache.insert(cache_key, cache_body).await;

    Ok(top_langs)
}

pub async fn get_waka_top_langs(
    State(waka_cache): State<Cache<String, String>>,
    Query(params): Query<Params>,
) -> Response {
    let username = params.username;
    let top_langs_res = get_top_langs_by_waka_intl(waka_cache, &username).await;
    if !top_langs_res.is_ok() {
        let message = top_langs_res.unwrap_err();
        let template = if message == "FailedGetStats" {
            templates::ErrorTemplate {
                first_line: "Failed to find a user.".to_string(),
                second_line: "Check if it’s spelled correctly".to_string(),
            }
        } else {
            templates::ErrorTemplate {
                first_line: "Failed to find a user languages.".to_string(),
                second_line: "Maybe he's inactive".to_string(),
            }
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

#[derive(Debug, Deserialize, Serialize)]
struct LanguageStat {
    name: String,
    color: String,
    percent: f32,
}

#[derive(Template)]
#[template(path = "compact/languages.html")]
pub struct CompactLanguagesTemplate {
    name: String,
    stats_bar: String,
    bar_legend: String,
}
