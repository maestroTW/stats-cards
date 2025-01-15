use std::collections::HashMap;

use askama::Template;
use axum::{
    extract,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    Json, Router,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;

lazy_static! {
    #[derive(Debug)]
    static ref LANG_TO_COLORS: HashMap<String, Value>  =
        serde_json::from_str(include_str!("../data/lang2hex.json")).unwrap();
}

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/v1/top-langs/wakatime", get(top_langs))
        .route("/v1/health", get(health));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7674")
        .await
        .unwrap();
    println!("🦀 Axum is running at {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct Health {
    version: String,
    status: String,
}

async fn health() -> impl IntoResponse {
    let data = Health {
        version: "0.1.0".to_string(),
        status: "ok".to_string(),
    };

    Json(data)
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    username: String,
}

async fn top_langs(extract::Query(params): extract::Query<Params>) -> impl IntoResponse {
    // println!("{:#?}", *LANG_TO_COLORS);
    let stats = [
        LanguageStat {
            name: "JavaScript".to_string(),
            color: "#F0DB4F".to_string(),
            percent: 26.52,
        },
        LanguageStat {
            name: "TypeScript".to_string(),
            color: "#007ACC".to_string(),
            percent: 16.82,
        },
        LanguageStat {
            name: "Python".to_string(),
            color: "#387EB8".to_string(),
            percent: 16.31,
        },
        LanguageStat {
            name: "Svelte".to_string(),
            color: "#FF3E00".to_string(),
            percent: 6.63,
        },
        LanguageStat {
            name: "Markdown".to_string(),
            color: "#42A5F5".to_string(),
            percent: 5.86,
        },
        LanguageStat {
            name: "JSON".to_string(),
            color: "#424445".to_string(),
            percent: 4.19,
        },
    ];

    let max_percent = stats.iter().fold(0.0, |acc, val| acc + val.percent);
    let max_width = 275.0;
    let mut bar_start_x = 20.0;
    let mut column_start_y = 93;

    let bar_data: Vec<String> = stats
        .iter()
        .map(|stat| {
            let stat_percent = stat.percent / max_percent;
            let block_width = max_width * stat_percent;
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
                    <text x="{text_x}" y="{text_y}" fill="#CAD3F5" class="stat-text">{1} {2}%</text>
                </g>
            "##,
                stat.color, stat.name, stat.percent,
            );

            column_start_y = if idx == 2 { 93 } else { column_start_y + 24 };

            element
        })
        .collect();

    let template = CompactLanguagesTemplate {
        name: params.username,
        stats_bar: bar_data.join("\n"),
        bar_legend: bar_legend.join("\n"),
    };
    SVGTemplate(template)
}

#[derive(Template)]
#[template(path = "compact/languages.html")]
struct CompactLanguagesTemplate {
    name: String,
    stats_bar: String,
    bar_legend: String,
}

#[derive(Debug)]
struct LanguageStat {
    name: String,
    color: String,
    percent: f32,
}

struct SVGTemplate<T>(T);

impl<T> IntoResponse for SVGTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => {
                let body = Html(html);

                ([("content-type", "image/svg+xml; charset=utf-8")], body).into_response()
            }
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}
