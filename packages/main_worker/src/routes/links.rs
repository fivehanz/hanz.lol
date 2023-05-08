use crate::types::links::{set_link, Link, LinkSlug};
// use crate::utils::links::{delete_link, get_link, set_link, update_link};
use axum::{extract, http::StatusCode, Json};

// create a new link
// TODO: implement creation of a new link
pub async fn create(extract::Json(payload): extract::Json<Link>) -> StatusCode {
    match set_link(payload.slug, payload.href) {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

// read and return the link
// TODO: implement reading of the link and returning the 'href'
pub async fn read(extract::Json(payload): extract::Json<LinkSlug>) -> (StatusCode, Json<Link>) {
    // ! write the logic of getting the link in utils files and import it
    // match get_link(payload) {
    //     Ok(link) => (StatusCode::OK, Json(payload)),
    //     Err(err) => (StatusCode::BAD_REQUEST, Json(err)),
    // }

    (
        StatusCode::OK,
        Json(Link {
            slug: payload.slug,
            href: "https://bio.hanz.lol".to_owned(),
        }),
    )
}

// TODO: implement updating the link 'href' based on 'key'
pub async fn update() -> StatusCode {
    StatusCode::OK
}

// TODO: implement deleting the link based on 'key'
pub async fn delete() -> StatusCode {
    StatusCode::OK
}
