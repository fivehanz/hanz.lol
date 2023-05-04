use axum::http::header::CONTENT_TYPE;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::get,
    Router as AxumRouter,
};
use axum_cloudflare_adapter::{to_axum_request, to_worker_response, EnvWrapper};
use std::fmt::Error;
use std::ops::Deref;
use tower_service::Service;
use worker::{event, Context, Env, Request, Response};

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
        .route("/:slug", get(slug_handler))
        .with_state(axum_state);

    let axum_request = to_axum_request(req).await.unwrap();
    let axum_response = _router.call(axum_request).await.unwrap();
    match to_worker_response(axum_response).await {
        Ok(response) => Ok(response),
        Err(_err) => Err(std::fmt::Error),
    }
}

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
