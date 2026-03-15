#![allow(
    clippy::module_name_repetitions,
    clippy::doc_link_with_quotes,
    clippy::redundant_pub_crate
)]

use std::sync::Arc;

use context::Context;
use tokio::sync::Mutex;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod authorization;
mod charger;
mod connector;
mod context;
mod control_app;
mod handlers;
mod ocpp;
mod ocpp_app;
mod provisioning;
mod rpc;
mod tests;

#[tokio::main]
async fn main() {
    //let _state = State {};
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "csms=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let context = Arc::new(Mutex::new(Context::new()));
    // build our ocpp app with some routes
    let ocpp_app = ocpp_app::init_router(context.clone());
    // build our control app with some routes
    let control_app = control_app::init_router(context);

    let ocpp_addr = "0.0.0.0:9000";
    let control_addr = "0.0.0.0:8000";

    tracing::debug!("ocpp listening on {ocpp_addr}");
    tracing::debug!("control listening on {control_addr}");

    let ocpp_listener = tokio::net::TcpListener::bind(ocpp_addr).await.unwrap();
    let controler_listener = tokio::net::TcpListener::bind(control_addr).await.unwrap();
    let ocpp_task = axum::serve(ocpp_listener, ocpp_app);
    let control_task = axum::serve(controler_listener, control_app);
    // let ocpp_task = axum::bind(&ocpp_addr).serve(ocpp_app.into_make_service());
    // let control_task = axum::Server::bind(&control_addr).serve(control_app.into_make_service());
    tokio::select! {
        res = ocpp_task => {
            if let Err(err) = res {
                tracing::error!("ocpp error {err:?}");
            }
        }
        res = control_task => {
            if let Err(err) = res {
                tracing::error!("control error {err:?}");
            }
        }
    };
}
