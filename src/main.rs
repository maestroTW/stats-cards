mod api;
mod data;
mod routes;
mod templates;

use axum::{routing::get, Router};
use dotenv::dotenv;
use moka::future::Cache;
use tower_http::services::ServeDir;

use crate::data::config::CONFIG;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cache: Cache<String, String> = Cache::builder()
        .time_to_live(CONFIG.cache_ttl)
        .max_capacity(16384)
        .build();

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(routes::index::get_index))
        .route(
            "/v1/top-langs/wakatime",
            get(routes::languages::get_waka_top_langs),
        )
        .route(
            "/v1/top-langs/github",
            get(routes::languages::get_github_top_langs),
        )
        .route(
            "/v1/activity/github",
            get(routes::activity::get_github_activity_graph),
        )
        .route("/v1/health", get(routes::health::get_health))
        .with_state(cache);

    let listener = tokio::net::TcpListener::bind(format!("{0}:{1}", CONFIG.hostname, CONFIG.port))
        .await
        .unwrap();
    println!("🦀 Axum is running at {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
