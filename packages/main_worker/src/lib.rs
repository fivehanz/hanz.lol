mod routes;
mod types;
mod utils;

use axum::{
    routing::{delete, get, post, put},
    Router as AxumRouter,
};
use axum_cloudflare_adapter::{to_axum_request, to_worker_response, EnvWrapper};
use routes::{
    links::{create, delete as delete_route, read, update},
    main::{fallback, home, slug_handler},
};
use std::fmt::Error;
use tower_service::Service;
use utils::axum::AxumState;
use worker::{event, Context, Env, Request, Response};

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response, Error> {
    // Initialize console error panic hook
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let axum_state = AxumState {
        env_wrapper: EnvWrapper::new(env),
    };

    let mut router: AxumRouter = AxumRouter::new()
        .route("/", get(home))
        .route("/:slug", get(slug_handler))
        .route("/links/get", post(read))
        .route("/links/create", post(create))
        .route("/links/update", put(update))
        .route("/links/delete", delete(delete_route))
        .with_state(axum_state)
        .fallback(fallback);

    let axum_request = to_axum_request(req).await.unwrap();
    let axum_response = router.call(axum_request).await.unwrap();
    match to_worker_response(axum_response).await {
        Ok(response) => Ok(response),
        Err(_err) => Err(std::fmt::Error),
    }
}
