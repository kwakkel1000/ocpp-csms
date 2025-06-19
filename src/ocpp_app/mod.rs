use axum::{routing::get, Router};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use ws::ws_connect;

pub mod server;
pub mod ws;

pub fn init_router() -> Router {
    Router::new().route("/:station_id", get(ws_connect)).layer(
        TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true)),
    )
}
