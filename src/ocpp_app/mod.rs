use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::sync::Mutex;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use ws::ws_connect;

use crate::context::Context;

pub mod server;
pub mod ws;

pub fn init_router(context: Arc<Mutex<Context>>) -> Router {
    tracing::info!("init router");
    Router::new()
        .route("/{station_id}", get(ws_connect))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .with_state(context)
}
