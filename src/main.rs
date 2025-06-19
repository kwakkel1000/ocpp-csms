#![allow(
    clippy::module_name_repetitions,
    clippy::doc_link_with_quotes,
    clippy::redundant_pub_crate
)]

use context::init;
use std::net::SocketAddr;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod authorization;
mod charger;
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
    init();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "csms=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our ocpp app with some routes
    let ocpp_app = ocpp_app::init_router();
    // build our control app with some routes
    let control_app = control_app::init_router();

    // run it with hyper
    let ocpp_addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    let control_addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::debug!("ocpp listening on {}", ocpp_addr);
    tracing::debug!("control listening on {}", control_addr);
    let ocpp_task = axum::Server::bind(&ocpp_addr).serve(ocpp_app.into_make_service());
    let control_task = axum::Server::bind(&control_addr).serve(control_app.into_make_service());
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

async fn get_status() {}
