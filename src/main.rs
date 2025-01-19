mod api;
mod routes;
mod templates;

use axum::{routing::get, Router};
use dotenv::dotenv;
use moka::future::Cache;
use std::time::Duration;

const CACHE_TTL: Duration = Duration::from_secs(7200);

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cache: Cache<String, String> = Cache::builder()
        .time_to_live(CACHE_TTL)
        .max_capacity(16384)
        .build();

    let app = Router::new()
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

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7674")
        .await
        .unwrap();
    println!("🦀 Axum is running at {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
