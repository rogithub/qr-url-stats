use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub id: String,
    pub short_url: String,
    pub qr_svg: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}