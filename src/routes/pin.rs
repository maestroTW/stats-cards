use std::vec;

use crate::api::huggingface;
use crate::prepared_templates::PreparedTemplate;
use crate::templates;
use crate::utils::utils::fmt_num;
use crate::{
    api::huggingface::{Model as HFModel, ModelResponse as HFModelResponse},
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
    likes: String,
    downloads: Option<&'a String>,
    icon: HFPinIcon,
    tags: Vec<HFTag>,
}

pub async fn get_huggingface_pin_model_impl(
    cache: Cache<String, String>,
    username: &String,
    repo: &String,
) -> Result<HFModel, PreparedTemplate> {
    if username.is_empty() || repo.is_empty() {
        return Err(PreparedTemplate::FailedFindRepo);
    }

    let cache_key = format!("huggingface:model:{username}:{repo}");
    if let Some(cached) = cache.get(&cache_key).await {
        let langs = serde_json::from_str(&cached).unwrap();
        return Ok(langs);
    }

    let model_res = huggingface::get_model(username, repo).await;
    if !model_res.is_ok() {
        return Err(PreparedTemplate::Unknown);
    }

    let model = match model_res.unwrap() {
        HFModelResponse::Failed(err) => {
            let err_template = match err.error.as_str() {
                "Invalid credentials in Authorization header" => PreparedTemplate::BadCredentials,
                "Invalid username or password." => PreparedTemplate::BadCredentials,
                "Repository not found" => PreparedTemplate::FailedFindRepo,
                _ => PreparedTemplate::Unknown,
            };
            return Err(err_template);
        }
        HFModelResponse::Valid(res) => res,
    };

    let cache_body = serde_json::to_string(&model).unwrap();
    cache.insert(cache_key, cache_body).await;

    Ok(model)
}

pub fn render_huggingface_pin(
    username: String,
    repo: String,
    show_owner: bool,
    pin_data: Result<HFModel, PreparedTemplate>,
) -> Response {
    if !pin_data.is_ok() {
        return pin_data.unwrap_err().render();
    }

    let data = pin_data.unwrap();
    let mut raw_tags: Vec<String> = vec![];
    if let Some(model_type) = data
        .config
        .as_ref()
        .and_then(|config| config.model_type.clone())
    {
        raw_tags.push(model_type);
    }

    if let Some(pipeline_tag) = data.pipeline_tag {
        raw_tags.push(pipeline_tag.to_string());
    }

    if let Some(license) = data.card_data.license {
        if license != "other" {
            raw_tags.push(license.to_uppercase());
        }
    }

    let icon = HFPinIcon::Model;
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

    let downloads = fmt_num(data.downloads as i32);
    let repo_text = if show_owner { data.id } else { repo.clone() };

    let template = HFPinTemplate {
        name: username,
        desc: repo,
        repo_text,
        downloads: Some(&downloads),
        likes: fmt_num(data.likes as i32),
        icon,
        tags,
    };

    let svg_template = templates::SVGTemplate(template);
    templates::SVGTemplate::<HFPinTemplate>::into_response(svg_template)
}

pub async fn get_huggingface_repo(
    State(cache): State<Cache<String, String>>,
    Query(params): Query<Params>,
) -> Response {
    let username = params.username;
    let repo = params.repo;
    let show_owner = if let Some(show_owner) = params.show_owner {
        show_owner
    } else {
        false
    };
    let model = get_huggingface_pin_model_impl(cache, &username, &repo).await;
    render_huggingface_pin(username, repo, show_owner, model)
}
