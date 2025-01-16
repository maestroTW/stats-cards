use reqwest::Error;
use serde::{Deserialize, Serialize};

macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Deserialize, Serialize)]
        #[allow(dead_code)]
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub enum WakaTimeRange {
    #[serde(rename = "last_7_days")]
    Last7Days,
    #[serde(rename = "last_30_days")]
    Last30Days,
    #[serde(rename = "last_6_months")]
    Last6Months,
    #[serde(rename = "last_year")]
    LastYear,
    #[serde(rename = "all_time")]
    AllTime,
}

// same for editors, categories, languages
pub_struct! { WakaTimeEntry {
    name: String,
    total_seconds: f64,
    percent: f32,
    digital: String,
    decimal: String,
    text: String,
    hours: i32,
    minutes: i8,
}}

pub_struct! {  WakaTimeStats {
    id: String,
    user_id: String,
    range: WakaTimeRange,
    timeout: i32,
    writes_only: bool,
    holidays: i32,
    status: String, // pending_update
    human_readable_daily_average: String,
    is_up_to_date: bool,
    total_seconds: f64,
    total_seconds_including_other_language: f64,
    percent_calculated: i8,
    days_minus_holidays: i32,
    daily_average_including_other_language: f64,
    human_readable_daily_average_including_other_language: String,
    editors: Vec<WakaTimeEntry>,
    is_up_to_date_pending_future: bool,
    is_already_updating: bool,
    categories: Vec<WakaTimeEntry>,
    languages: Vec<WakaTimeEntry>,
    is_stuck: bool,
    daily_average: f64,
    human_readable_total_including_other_language: String,
    days_including_holidays: i32,
    operating_systems: Vec<WakaTimeEntry>,
    human_readable_total: String,
    is_cached: bool,
    username: String,
    is_including_today: bool,
    human_readable_range: String,
    is_coding_activity_visible: bool,
    is_language_usage_visible: bool,
    is_editor_usage_visible: bool,
    is_category_usage_visible: bool,
    is_os_usage_visible: bool,
}}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct WakaTimeResponse<T> {
    pub data: T,
}

pub async fn get_stats(username: &String) -> Result<WakaTimeResponse<WakaTimeStats>, Error> {
    let request_url = format!("https://wakatime.com/api/v1/users/{username}/stats/all_time");
    let stats = reqwest::get(&request_url)
        .await?
        .json::<WakaTimeResponse<WakaTimeStats>>()
        .await?;

    Ok(stats)
}
