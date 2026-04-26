use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct ContactForm {
    #[validate(length(min = 1, max = 100))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 1, max = 5000))]
    pub message: String,
}

#[derive(Serialize)]
pub struct SuccessResponse {
    pub status: &'static str,
    pub message: &'static str,
}

#[derive(Deserialize)]
pub struct CliExecuteRequest {
    pub command: String,
}

#[derive(Serialize)]
pub struct CliExecuteResponse {
    pub output: String,
}

#[derive(Deserialize)]
pub struct CliAutocompleteRequest {
    pub partial: String,
}

#[derive(Serialize)]
pub struct CliAutocompleteResponse {
    pub matches: Vec<String>,
}
