mod api;
mod routes;
mod templates;

use axum::{routing::get, Router};
use moka::future::Cache;
use std::time::Duration;

const CACHE_TTL: Duration = Duration::from_secs(7200);

#[tokio::main]
async fn main() {
    let waka_cache: Cache<String, String> = Cache::builder()
        .time_to_live(CACHE_TTL)
        .max_capacity(4096)
        .build();

    let app = Router::new()
        .route(
            "/v1/top-langs/wakatime",
            get(routes::languages::get_waka_top_langs),
        )
        .route("/v1/health", get(routes::health::get_health))
        .with_state(waka_cache);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7674")
        .await
        .unwrap();
    println!("🦀 Axum is running at {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
