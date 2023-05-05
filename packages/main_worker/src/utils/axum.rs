use axum_cloudflare_adapter::EnvWrapper;

#[derive(Clone)]
pub struct AxumState {
    pub env_wrapper: EnvWrapper,
}
