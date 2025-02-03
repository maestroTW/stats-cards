use crate::templates::HtmlTemplate;

use askama::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

pub async fn get_index() -> impl IntoResponse {
    let template = IndexTemplate {};
    let html_template = HtmlTemplate(template);
    html_template.into_response()
}
