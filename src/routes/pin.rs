use std::vec;

use crate::api::huggingface::{self};
use crate::data::config::CONFIG;
use crate::data::theme::{Theme, ThemeData};
use crate::prepared_templates::PreparedTemplate;
use crate::templates;
use crate::{
    api::huggingface::{
        DatasetResponse as HFDatasetResponse, ErrorResponse as HFErrorResponse,
        ModelResponse as HFModelResponse, RepoData as HFRepoData, RepoResponse as HFRepoResponse,
        SpaceResponse as HFSpaceResponse,
    },
    utils::svg::calc_width,
};

use askama::Template;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
};
use moka::future::Cache;
use serde::{Deserialize, Serialize};

const MAX_PIN_WIDTH: usize = 400;

#[derive(Deserialize, Serialize)]
pub struct Params {
    username: String,
    repo: String,
    theme: Option<Theme>,
    #[serde(rename = "type")]
    typename: HFPinIcon,
    show_owner: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum HFPinIcon {
    #[serde(rename = "model")]
    Model,
    #[serde(rename = "dataset")]
    Dataset,
    #[serde(rename = "space")]
    Space,
}

impl PartialEq<&str> for HFPinIcon {
    fn eq(&self, other: &&str) -> bool {
        match self {
            HFPinIcon::Model => *other == "model",
            HFPinIcon::Dataset => *other == "dataset",
            HFPinIcon::Space => *other == "space",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HFTag {
    name: String,
    width: usize,
    translate_x: usize,
    visible: bool,
}

#[derive(Template)]
#[template(path = "compact/pin/huggingface.html")]
pub struct HFPinTemplate<'a> {
    name: String,
    desc: String,
    repo_text: String,
    likes: &'a String,
    downloads: Option<&'a String>,
    icon: HFPinIcon,
    tags: Vec<HFTag>,
    theme_data: ThemeData,
}

fn hf_handle_error_template(err: HFErrorResponse) -> PreparedTemplate {
    match err.error.as_str() {
        "Invalid credentials in Authorization header" => PreparedTemplate::BadCredentials,
        "Invalid username or password." => PreparedTemplate::BadCredentials,
        "Repository not found" => PreparedTemplate::FailedFindRepo,
        _ => PreparedTemplate::Unknown,
    }
}

pub async fn huggingface_get_data(
    username: &String,
    repo: &String,
    typename: &HFPinIcon,
) -> Result<HFRepoData, PreparedTemplate> {
    let data = match typename {
        HFPinIcon::Model => huggingface::get_model(username, repo)
            .await
            .map(HFRepoResponse::Model),
        HFPinIcon::Dataset => huggingface::get_dataset(username, repo)
            .await
            .map(HFRepoResponse::Dataset),
        HFPinIcon::Space => huggingface::get_space(username, repo)
            .await
            .map(HFRepoResponse::Space),
    };
    if !data.is_ok() {
        return Err(PreparedTemplate::Unknown);
    }

    match data.unwrap() {
        HFRepoResponse::Model(HFModelResponse::Valid(res)) => Ok(HFRepoData::Model(res)),
        HFRepoResponse::Dataset(HFDatasetResponse::Valid(res)) => Ok(HFRepoData::Dataset(res)),
        HFRepoResponse::Space(HFSpaceResponse::Valid(res)) => Ok(HFRepoData::Space(res)),
        HFRepoResponse::Model(HFModelResponse::Failed(err))
        | HFRepoResponse::Space(HFSpaceResponse::Failed(err))
        | HFRepoResponse::Dataset(HFDatasetResponse::Failed(err)) => {
            Err(hf_handle_error_template(err))
        }
    }
}

pub async fn get_huggingface_pin_impl(
    cache: Cache<String, String>,
    username: &String,
    repo: &String,
    typename: &HFPinIcon,
) -> Result<HFRepoData, PreparedTemplate> {
    if username.is_empty() || repo.is_empty() {
        return Err(PreparedTemplate::FailedFindRepo);
    }

    let cache_key = format!("huggingface:{:?}:{username}:{repo}", typename);
    if let Some(cached) = cache.get(&cache_key).await {
        let langs = serde_json::from_str(&cached).unwrap();
        return Ok(langs);
    }

    let data = huggingface_get_data(username, repo, typename).await;
    let result = match data {
        Ok(data) => data,
        Err(err) => return Err(err),
    };

    let cache_body = serde_json::to_string(&result).unwrap();
    cache.insert(cache_key, cache_body).await;

    Ok(result)
}

pub fn render_huggingface_pin(
    username: String,
    repo: String,
    show_owner: bool,
    theme: Theme,
    pin_data: Result<HFRepoData, PreparedTemplate>,
) -> Response {
    let raw_data = match pin_data {
        Ok(data) => data,
        Err(err) => return err.render(),
    };

    let mut raw_tags: Vec<String> = vec![];
    let downloads_raw: &Option<String> = &raw_data.get_downloads_count();
    let downloads: Option<&String> = downloads_raw.as_ref();
    if let HFRepoData::Model(model) = &raw_data {
        if let Some(model_type) = model
            .config
            .as_ref()
            .and_then(|config| config.model_type.clone())
        {
            raw_tags.push(model_type);
        }

        if let Some(pipeline_tag) = &model.pipeline_tag {
            raw_tags.push(pipeline_tag.to_string());
        }
    }

    if let HFRepoData::Dataset(dataset) = &raw_data {
        if let Some(task_category) = &dataset
            .base
            .card_data
            .task_categories
            .as_ref()
            .and_then(|categories| categories.first())
        {
            raw_tags.push((*task_category).clone())
        }
    }

    if let HFRepoData::Space(space) = &raw_data {
        let running_on = space.runtime.hardware.current.to_string();
        raw_tags.push(format!("Running on {running_on}"));
    }

    let icon = match &raw_data {
        HFRepoData::Model(_) => HFPinIcon::Model,
        HFRepoData::Dataset(_) => HFPinIcon::Dataset,
        HFRepoData::Space(_) => HFPinIcon::Space,
    };

    if let Some(license) = &raw_data.get_license() {
        if license != "other" {
            raw_tags.push(license.to_uppercase());
        }
    }

    let likes = &raw_data.get_likes();
    if raw_tags.len() == 0 {
        if let Some(tag) = &raw_data.get_repo_tags().first() {
            raw_tags.push((*tag).clone());
        }
    }

    let mut translate_x: usize = 0;
    let tags: Vec<HFTag> = raw_tags
        .iter()
        .map(|tag| {
            let width = calc_width(tag, 13.0) + 8;
            let tag_translate_x = translate_x;
            translate_x += width + 10;
            return HFTag {
                name: tag.clone(),
                width,
                translate_x: tag_translate_x,
                visible: width + tag_translate_x <= MAX_PIN_WIDTH,
            };
        })
        .filter(|tag| tag.visible)
        .collect();

    let repo_text = if show_owner {
        raw_data.get_id()
    } else {
        repo.clone()
    };

    let theme_data = theme.get_data();
    let template = HFPinTemplate {
        name: username,
        desc: repo,
        repo_text,
        downloads,
        likes,
        icon,
        tags,
        theme_data,
    };

    let svg_template = templates::SVGTemplate(template);
    templates::SVGTemplate::<HFPinTemplate>::into_response(svg_template)
}

pub async fn get_huggingface_pin(
    State(cache): State<Cache<String, String>>,
    Query(params): Query<Params>,
) -> Response {
    let theme = params.theme.unwrap_or(CONFIG.default_theme.clone());
    let username = params.username;
    let repo = params.repo;
    let typename = params.typename;
    let show_owner = if let Some(show_owner) = params.show_owner {
        show_owner
    } else {
        false
    };
    let repo_data = get_huggingface_pin_impl(cache, &username, &repo, &typename).await;
    render_huggingface_pin(username, repo, show_owner, theme, repo_data)
}
