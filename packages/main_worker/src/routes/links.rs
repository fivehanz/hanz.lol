use axum::{extract, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Link {
    slug: String,
    href: String,
}

#[derive(Deserialize)]
pub struct LinkSlug {
    slug: String,
}

// create a new link
pub async fn create(extract::Json(payload): extract::Json<Link>) -> (StatusCode, Json<Link>) {
    // return the payload for now
    (StatusCode::CREATED, Json(payload))
}

// read and return the link
pub async fn read(extract::Json(_payload): extract::Json<LinkSlug>) -> (StatusCode, Json<Link>) {
    (
        StatusCode::OK,
        Json(Link {
            slug: _payload.slug,
            href: "https://bio.hanz.lol".to_owned(),
        }),
    )
}

pub async fn update() -> StatusCode {
    StatusCode::OK
}

pub async fn delete() -> StatusCode {
    StatusCode::OK
}
