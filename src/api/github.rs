use axum::http::{HeaderMap, HeaderValue};
use lazy_static::lazy_static;
use reqwest::Error;
use reqwest::{
    Client,
    header::{ACCEPT, AUTHORIZATION, USER_AGENT},
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::data::config::CONFIG;

lazy_static! {
    static ref REQ_CLIENT: Client = Client::new();
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Repo {
    pub private: bool,
    pub language: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributionDay {
    pub weekday: i8,
    pub date: String,
    pub contribution_count: i32,
    pub color: String, // hex
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributionWeek {
    pub contribution_days: Vec<ContributionDay>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContributionMonth {
    pub name: String,
    pub year: i16,
    pub first_day: String,
    pub total_weeks: i8,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GithubCalendar {
    pub total_contributions: i32,
    pub weeks: Vec<ContributionWeek>,
    pub months: Vec<ContributionMonth>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Contributions {
    pub contribution_calendar: GithubCalendar,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub created_at: String,
    pub contributions_collection: Contributions,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserData {
    // user or null if not found
    pub user: Option<User>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuccessResponse<T> {
    pub data: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ActivityResponse {
    Failed(ErrorResponse),
    Valid(SuccessResponse<UserData>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum GithubStatsResponse {
    Failed(ErrorResponse),
    Valid(Vec<Repo>),
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

pub async fn get_stats(username: &String) -> Result<GithubStatsResponse, Error> {
    let request_url = format!("https://api.github.com/users/{username}/repos");
    let mut headers = get_headers();
    headers.insert(
        "X-GitHub-Api-Version",
        HeaderValue::from_str("2022-11-28").unwrap(),
    );

    let res = REQ_CLIENT.get(&request_url).headers(headers).send().await?;
    let data = res.json::<GithubStatsResponse>().await?;

    Ok(data)
}

// max 365 days
#[allow(dead_code)]
pub async fn get_activity(
    username: &String,
    start_date: &String,
    end_date: &String,
) -> Result<ActivityResponse, Error> {
    let request_url = format!("https://api.github.com/graphql");
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

    let data = REQ_CLIENT
        .post(&request_url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await?
        .json::<ActivityResponse>()
        .await?;

    Ok(data)
}
