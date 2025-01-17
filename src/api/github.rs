use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubRepo {
    pub private: bool,
    pub language: Option<String>,
}

pub async fn get_stats(username: &String) -> Result<Vec<GithubRepo>, Error> {
    let request_url = format!("https://api.github.com/users/{username}/repos");
    let client = reqwest::Client::new();
    let github_token = match std::env::var("GITHUB_TOKEN") {
        Ok(val) => {
            if val.len() > 0 {
                format!("Bearer {val}")
            } else {
                "".to_string()
            }
        }
        Err(_) => "".to_string(),
    };

    // github api doesn't like default reqwest user-agent
    let stats = client
        .get(&request_url)
        .header(ACCEPT, "application/vnd.github+json")
        .header(AUTHORIZATION, &github_token)
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:135.0) Gecko/20100101 Firefox/135.0",
        )
        .send()
        .await?
        .json::<Vec<GithubRepo>>()
        .await?;

    Ok(stats)
}
