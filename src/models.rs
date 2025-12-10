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

#[derive(Deserialize)]
pub struct LocationRequest {
    pub lat: f64,
    pub lon: f64,
    pub description: Option<String>, 
}

#[derive(Serialize)]
pub struct LocationResponse {
    pub message: String,
    pub location_id: i64,
}