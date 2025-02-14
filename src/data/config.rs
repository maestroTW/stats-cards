use lazy_static::lazy_static;
use serde::Deserialize;
use std::{env, time::Duration};

#[derive(Deserialize)]
pub struct CargoConfig {
    package: CargoPackage,
}

#[derive(Deserialize)]
pub struct CargoPackage {
    version: String,
}

pub struct Config {
    pub version: String,
    pub hostname: String,
    pub port: u16,
    pub cache_ttl: Duration,
    pub user_agent: String,
    pub github_token: String,
}

lazy_static! {
    static ref CARGO_CONFIG: CargoConfig =
        toml::from_str(include_str!("../../Cargo.toml")).unwrap();
    pub static ref CONFIG: Config = Config {
        version: CARGO_CONFIG.package.version.clone(),
        hostname: match env::var("SERVICE_HOST") {
            Ok(host) => host,
            Err(_) => "127.0.0.1".to_string(),
        },
        port: match env::var("SERVICE_PORT") {
            Ok(port) => port.parse().unwrap(),
            Err(_) => 7674,
        },
        cache_ttl: Duration::from_secs(7200),
        user_agent:
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:135.0) Gecko/20100101 Firefox/135.0"
                .to_string(),
        github_token: std::env::var("GITHUB_TOKEN")
            .ok()
            .filter(|val| !val.is_empty())
            .map(|val| format!("Bearer {val}"))
            .unwrap_or_default()
    };
}
