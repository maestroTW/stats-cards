use std::str::FromStr;

use axum::http::{HeaderMap, HeaderName, HeaderValue};
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
pub struct UserActivity {
    pub created_at: String,
    pub contributions_collection: Contributions,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageNode {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageEdge {
    pub size: i32,
    pub node: LanguageNode,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RepositoryNodeLanguages {
    pub edges: Vec<LanguageEdge>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RepositoryNode {
    pub name: String,
    pub languages: RepositoryNodeLanguages,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLanguagesRepositories {
    pub nodes: Vec<RepositoryNode>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserLanguages {
    pub repositories: UserLanguagesRepositories,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OptionUserData<T> {
    // user or null if not found
    pub user: Option<T>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Repository {
    pub name: String,
    pub description: Option<String>,
    pub language: Option<String>,
    pub stargazers_count: u32,
    pub forks_count: u32,
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
pub enum GraphQLResponse<T> {
    Failed(ErrorResponse),
    Valid(SuccessResponse<T>),
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RestResponse<T> {
    Failed(ErrorResponse),
    Valid(T),
}

pub type ActivityResponse = GraphQLResponse<OptionUserData<UserActivity>>;
pub type LanguagesResponse = GraphQLResponse<OptionUserData<UserLanguages>>;
pub type RepositoryResponse = RestResponse<Repository>;

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

pub async fn request_graphql<T: for<'de> Deserialize<'de>>(
    graphql_query: &String,
) -> Result<T, Error> {
    let request_url = format!("https://api.github.com/graphql");
    let headers = get_headers();
    let request_body = json!({
        "query": graphql_query
    });

    let data = REQ_CLIENT
        .post(&request_url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await?
        .json::<T>()
        .await?;

    Ok(data)
}

pub async fn request_get_api<T: for<'de> Deserialize<'de>>(pathname: &str) -> Result<T, Error> {
    let request_url = format!("https://api.github.com{pathname}");
    let mut headers = get_headers();
    headers.insert(
        HeaderName::from_str("X-GitHub-Api-Version").unwrap(),
        HeaderValue::from_str("2022-11-28").unwrap(),
    );

    let data = REQ_CLIENT
        .get(&request_url)
        .headers(headers)
        .send()
        .await?
        .json::<T>()
        .await?;

    Ok(data)
}

#[allow(dead_code)]
pub async fn get_languages(username: &String) -> Result<LanguagesResponse, Error> {
    let graphql_query = format!(
        r###"query {{
            user(login: "{username}") {{
                repositories(ownerAffiliations: OWNER, isFork: false, first: 100, orderBy: {{field: STARGAZERS, direction: DESC}}) {{
                    nodes {{
                        name
                        languages(first: 10, orderBy: {{field: SIZE, direction: DESC}}) {{
                            edges {{
                                size
                                node {{
                                    name
                                }}
                            }}
                        }}
                    }}
                }}
            }}
        }}"###
    );

    request_graphql::<LanguagesResponse>(&graphql_query).await
}

// max 365 days
pub async fn get_activity(
    username: &String,
    start_date: &String,
    end_date: &String,
) -> Result<ActivityResponse, Error> {
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

    request_graphql::<ActivityResponse>(&graphql_query).await
}

pub async fn get_repo(username: &String, repo_name: &String) -> Result<RepositoryResponse, Error> {
    let pathname = format!("/repos/{username}/{repo_name}");
    request_get_api::<RepositoryResponse>(&pathname).await
}
