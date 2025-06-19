use axum::{
    extract::{Path, WebSocketUpgrade},
    response::IntoResponse,
};
use tokio::sync::mpsc;

use crate::{
    context::get_context,
    ocpp_app::server::{handle_error, handle_socket, validate_station},
};

pub async fn ws_connect(Path(station): Path<String>, ws: WebSocketUpgrade) -> impl IntoResponse {
    tracing::info!("Incoming connection from station {}", station);
    let name = station.clone();
    let (tx, rx) = mpsc::channel(1);
    get_context().add_charger(name.clone(), tx).await;

    match validate_station(&station) {
        Ok(()) => ws.on_upgrade(|socket| handle_socket(socket, name, rx)),
        Err(_) => ws.on_upgrade(handle_error),
    }
}
