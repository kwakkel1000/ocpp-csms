use std::sync::Arc;

use axum::{
    extract::{Path, State, WebSocketUpgrade},
    response::IntoResponse,
};
use tokio::sync::{Mutex, mpsc};

use crate::{
    context::Context,
    ocpp_app::server::{handle_error, handle_socket, validate_station},
};

pub async fn ws_connect(
    Path(station): Path<String>,
    ws: WebSocketUpgrade,
    State(context): State<Arc<Mutex<Context>>>,
) -> impl IntoResponse {
    tracing::info!("Incoming connection from station {}", station);
    let context_clone = context.clone();
    let name = station.clone();
    let (tx, rx) = mpsc::channel(1);
    context.lock().await.add_charger(name.clone(), tx);

    match validate_station(&station) {
        Ok(()) => ws.on_upgrade(|socket| {
            handle_socket(socket, name, rx, axum::extract::State(context_clone))
        }),
        Err(_) => ws.on_upgrade(handle_error),
    }
}
