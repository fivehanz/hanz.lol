use axum::http::header::CONTENT_TYPE;
use axum::{extract::State, response::IntoResponse, routing::get, Router as AxumRouter};
use axum_cloudflare_adapter::{to_axum_request, to_worker_response, EnvWrapper};
use std::fmt::Error;
use std::ops::Deref;
use tower_service::Service;
use worker::{console_log, event, Context, Env, Request, Response, Var};

#[derive(Clone)]
pub struct AxumState {
    pub env_wrapper: EnvWrapper,
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response, Error> {
    let axum_state = AxumState {
        env_wrapper: EnvWrapper::new(env),
    };

    let mut _router: AxumRouter = AxumRouter::new()
        .route("/", get(home))
        .with_state(axum_state);

    let axum_request = to_axum_request(req).await.unwrap();
    let axum_response = _router.call(axum_request).await.unwrap();
    match to_worker_response(axum_response).await {
        Ok(response) => Ok(response),
        Err(_err) => Err(std::fmt::Error),
    }
}

// default home route handler
pub async fn home(State(state): State<AxumState>) -> impl IntoResponse {
    let env: &Env = state.env_wrapper.env.deref();
    let worker_rs_version: Var = env.var("WORKERS_RS_VERSION").unwrap();

    console_log!("WORKERS_RS_VERSION: {}", worker_rs_version.to_string());

    let body_text = String::from(
        "Hey, if you can see this -- cloudflare workers is working with axum -- Hanz.",
    );

    axum::response::Response::builder()
        .header(CONTENT_TYPE, "text/plain")
        .body(body_text)
        .unwrap()
}
