use axum::http::{HeaderMap, HeaderValue};
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::data::config::CONFIG;

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubRepo {
    pub private: bool,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubContributionDay {
    pub weekday: i8,
    pub date: String,
    pub contribution_count: i32,
    pub color: String, // hex
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubContributionWeek {
    pub contribution_days: Vec<GithubContributionDay>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubContributionMonth {
    pub name: String,
    pub year: i16,
    pub first_day: String,
    pub total_weeks: i8,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubCalendar {
    pub total_contributions: i32,
    pub weeks: Vec<GithubContributionWeek>,
    pub months: Vec<GithubContributionMonth>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubContributions {
    pub contribution_calendar: GithubCalendar,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubUser {
    pub created_at: String,
    pub contributions_collection: GithubContributions,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubUserData {
    // user or null if not found
    pub user: Option<GithubUser>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubRes<T> {
    pub data: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubFailedRes {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GithubActivityResponse {
    Failed(GithubFailedRes),
    Valid(GithubRes<GithubUserData>),
}

#[derive(Serialize)]
pub struct GraphQLReq {
    query: String,
}

pub fn get_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_str("application/vnd.github+json").unwrap(),
    );
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str(&CONFIG.user_agent).unwrap(),
    );
    if !&CONFIG.github_token.is_empty() {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&CONFIG.github_token).unwrap(),
        );
    }

    headers
}

pub async fn get_stats(username: &String) -> Result<Vec<GithubRepo>, Error> {
    let request_url = format!("https://api.github.com/users/{username}/repos");
    let client = reqwest::Client::new();
    let mut headers = get_headers();
    headers.insert(
        "X-GitHub-Api-Version",
        HeaderValue::from_str("2022-11-28").unwrap(),
    );

    let data = client
        .get(&request_url)
        .headers(headers)
        .send()
        .await?
        .json::<Vec<GithubRepo>>()
        .await?;

    Ok(data)
}

// max 365 days
#[allow(dead_code)]
pub async fn get_activity(
    username: &String,
    start_date: &String,
    end_date: &String,
) -> Result<GithubActivityResponse, Error> {
    let request_url = format!("https://api.github.com/graphql");
    let client = reqwest::Client::new();
    let headers = get_headers();

    let graphql_query = format!(
        r###"query {{
            user(login: "{username}") {{
                createdAt
                contributionsCollection(from: "{start_date}", to: "{end_date}") {{
                    contributionCalendar {{
                        totalContributions
                        weeks {{
                            contributionDays {{
                                weekday
                                date
                                contributionCount
                                color
                            }}
                        }}
                        months {{
                            name
                            year
                            firstDay
                            totalWeeks
                        }}
                    }}
                }}
            }}
        }}"###
    );
    let request_body = json!({
        "query": graphql_query
    });

    let data = client
        .post(&request_url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await?
        .json::<GithubActivityResponse>()
        .await?;

    Ok(data)
}
