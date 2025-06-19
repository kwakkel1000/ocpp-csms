use axum::{routing::get, Router};
use charge_profile::post_profile;
use start::post_start;
use status::get_status;
use stop::post_stop;

pub mod charge_profile;
pub mod start;
pub mod status;
pub mod stop;

pub fn init_router() -> Router {
    Router::new()
        .route("/start", get(get_404).post(post_start))
        .route("/stop", get(get_404).post(post_stop))
        .route("/profile", get(get_404).post(post_profile))
        // .route("/pause", get(get_404).post(post_pause))
        // .route("/resume", get(get_404).post(post_resume))
        .route("/status", get(get_status))
}

pub async fn get_404() {}
