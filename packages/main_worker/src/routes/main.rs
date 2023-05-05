use crate::utils::axum::AxumState;
use axum::http::header::CONTENT_TYPE;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::ops::Deref;
use worker::Env;

// #[worker_route_compat]
pub async fn home(State(state): State<AxumState>) -> impl IntoResponse {
    let _env: &Env = state.env_wrapper.env.deref();
    // let worker_rs_version: Var = env.var("WORKERS_RS_VERSION").unwrap();

    // console_log!("WORKERS_RS_VERSION: {}", worker_rs_version.to_string());

    let body_text = String::from(
        "Hey, if you can see this -- cloudflare workers is working with axum -- Hanz.",
    );

    axum::response::Response::builder()
        .header(CONTENT_TYPE, "text/plain")
        .body(body_text)
        .unwrap()
}

// slug handler
pub async fn slug_handler(Path(slug): Path<String>) -> impl IntoResponse {
    axum::response::Response::builder()
        .header(CONTENT_TYPE, "text/plain")
        .body(slug)
        .unwrap()

    // TODO: add logging, metrics
    // error handling with match
    // match database_init().await {
    // TODO: redirect to the relevant URL
    // Ok(db) => {
    //     match get_url_from_db(slug, &db).await {
    //         Ok(url) => Redirect::temporary(url.as_str()),
    //         Err(e) => {
    //             println!("{}", e);
    //             Redirect::temporary("/")
    //         }
    //     }
    // },

    // redirect to home
    // Err(e) => {
    //     println!("{} -- {}", e, slug);
    //     Redirect::temporary("/")
    // }
    // }
}

pub async fn fallback() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({ "status": "Not Found" })),
    )
}
