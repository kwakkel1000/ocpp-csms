use axum::extract::ws::Message;
use axum::extract::ws::WebSocket;
use axum::response::IntoResponse;
use axum::response::Response;
use futures_util::SinkExt;
use futures_util::StreamExt;
use tokio::sync::mpsc;

use crate::charger::ChargerMspcType;
use crate::context::get_context;
use crate::handlers::message::parse;

pub async fn handle_socket(
    socket: WebSocket,
    name: String,
    mut rx: mpsc::Receiver<ChargerMspcType>,
) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = socket.split();
    let (inner_tx, mut inner_rx) = mpsc::channel(1);
    let inner_tx_clone = inner_tx.clone();
    let name_clone = name.clone();
    let mut recv_task = tokio::spawn(async move {
        loop {
            if let Some(msg) = receiver.next().await {
                if let Ok(msg) = msg {
                    match &msg {
                        Message::Text(_msg) => {
                            let response = match parse(msg, &name_clone).await {
                                Ok(response) => response,
                                Err(err) => {
                                    tracing::error!("parse error {err:?}");
                                    continue;
                                }
                            };
                            if let Some(response) = response {
                                if let Err(err) = inner_tx_clone.send(response).await {
                                    // if let Err(err) = sender.send(response).await {
                                    tracing::error!("socket send error {err:?}");
                                    continue;
                                }
                            }
                        }
                        Message::Binary(_) => {
                            tracing::info!("client sent binary data");
                        }
                        Message::Ping(_) => {
                            tracing::info!("socket ping");
                        }
                        Message::Pong(_) => {
                            tracing::info!("socket pong");
                        }
                        Message::Close(_) => {
                            tracing::info!("client disconnected");
                            return;
                        }
                    }
                } else {
                    tracing::info!("client disconnected");
                    return;
                }
            }
        }
    });

    // Spawn the first task that will receive broadcast messages and send text
    // messages over the websocket to our client.
    let inner_tx_clone = inner_tx.clone();
    let mut send_task = tokio::spawn(async move {
        while let Some(request) = rx.recv().await {
            println!("request: {request:#?}");
            let stringified_request = match serde_json::to_string(&request) {
                Ok(request) => request,
                Err(err) => {
                    tracing::error!("serde_json::to_string error {err:?}");
                    break;
                }
            };
            let request = Message::Text(stringified_request);
            // In any websocket error, break loop.
            if inner_tx_clone.send(request).await.is_err() {
                // if sender.send(request).await.is_err() {
                break;
            }
        }
    });
    let mut mpsc_recv_task = tokio::spawn(async move {
        while let Some(text) = inner_rx.recv().await {
            println!("sending: {text:#?}");
            if sender.send(text).await.is_err() {
                break;
            }
        }
    });
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
        _ = &mut mpsc_recv_task => send_task.abort(),
    };
    get_context().del_charger(&name).await;
}

pub async fn handle_error(mut socket: WebSocket) {
    if socket
        .send(Message::Text(String::from("Not a valid station")))
        .await
        .is_err()
    {
        tracing::info!("client disconnected");
        return;
    }
    // close socket
    tracing::info!("closing socket due to invalid station");
    let _ = socket.close().await;
}

pub fn validate_station(station: &str) -> Result<(), ErrorUnknownStationId> {
    if station == "wallbox" {
        Ok(())
    } else {
        Err(ErrorUnknownStationId {})
    }
}

pub struct ErrorUnknownStationId {}

impl IntoResponse for ErrorUnknownStationId {
    fn into_response(self) -> Response {
        Response::default()
    }
}
