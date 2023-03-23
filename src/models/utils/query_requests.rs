use serde::Deserialize;

#[derive(Deserialize)]
pub struct SearchRequest {
    pub request: String,
}
