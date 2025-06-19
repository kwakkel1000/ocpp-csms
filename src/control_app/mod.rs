use axum::{routing::get, Router};
use start::post_start;
use stop::post_stop;

use crate::get_status;

pub mod start;
pub mod stop;

pub fn init_router() -> Router {
    Router::new()
        .route("/start", get(get_404).post(post_start))
        .route("/stop", get(get_404).post(post_stop))
        // .route("/pause", get(get_404).post(post_pause))
        // .route("/resume", get(get_404).post(post_resume))
        .route("/status", get(get_status))
}

pub async fn get_404() {}
