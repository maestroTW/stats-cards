use crate::templates::{ErrorTemplate, SVGTemplate};

use axum::response::{IntoResponse, Response};

#[derive(Debug)]
pub enum PreparedTemplate {
    FailedFindUser,
    FailedFindRepo,
    FailedFindLanguages,
    BadCredentials,
    APIRateLimit,
    Unknown,
}

impl PreparedTemplate {
    pub fn render(&self) -> Response {
        let template = match self {
            PreparedTemplate::FailedFindUser => ErrorTemplate {
                first_line: "Failed to find a user.",
                second_line: "Check if it’s spelled correctly",
            },
            PreparedTemplate::FailedFindRepo => ErrorTemplate {
                first_line: "Failed to find a repo.",
                second_line: "Check if it’s spelled correctly",
            },
            PreparedTemplate::FailedFindLanguages => ErrorTemplate {
                first_line: "Failed to find a user languages.",
                second_line: "Maybe he's inactive",
            },
            PreparedTemplate::BadCredentials => ErrorTemplate {
                first_line: "Bad credentials.",
                second_line: "Problems with service API token",
            },
            PreparedTemplate::APIRateLimit => ErrorTemplate {
                first_line: "Failed to fetch.",
                second_line: "Maybe our API ratelimited :(",
            },
            PreparedTemplate::Unknown => ErrorTemplate {
                first_line: "Unknown API error.",
                second_line: "Let us know about it",
            },
        };

        let svg_template = SVGTemplate(template);
        return SVGTemplate::<ErrorTemplate>::into_response(svg_template);
    }
}
