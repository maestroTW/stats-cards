use axum::{response::IntoResponse, Json};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CargoConfig {
    package: CargoPackage,
}

#[derive(Deserialize)]
pub struct CargoPackage {
    version: String,
}

lazy_static! {
    #[derive(Debug)]
    static ref CARGO_CONFIG: CargoConfig = toml::from_str(include_str!("../../Cargo.toml")).unwrap();
}

#[derive(Serialize)]
pub struct Health {
    version: String,
    status: String,
}

pub async fn get_health() -> impl IntoResponse {
    let data = Health {
        version: CARGO_CONFIG.package.version.to_string(),
        status: "ok".to_string(),
    };

    Json(data)
}
