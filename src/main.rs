#![allow(
    clippy::module_name_repetitions,
    clippy::doc_link_with_quotes,
    clippy::redundant_pub_crate
)]

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        // State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use rpc::{
    enums::{RemoteStartTransactionKind, RemoteStopTransactionKind},
    messages::OcppCall,
};
use rust_ocpp::v1_6::messages::{
    remote_start_transaction::RemoteStartTransactionRequest,
    remote_stop_transaction::RemoteStopTransactionRequest,
};
use std::collections::HashMap;
use std::sync::Arc;

use axum::extract::Path;
use axum::response::Response;
use handlers::message::parse;
//use state::State;
use once_cell::sync::OnceCell;
use std::net::SocketAddr;

//use crate::rpc::messages::OcppMessageType;
use tokio::sync::{mpsc, Mutex};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod authorization;
mod handlers;
mod ocpp;
mod provisioning;
mod rpc;
//mod security;
//mod state;
mod tests;

static CONTEXT: OnceCell<Arc<Context>> = OnceCell::new();

type ChargerMspcType = OcppCall;
#[derive(Clone)]
struct Charger {
    pub _name: String,
    pub tx: mpsc::Sender<ChargerMspcType>,
    pub transaction_id: Option<i32>,
}

impl Charger {
    pub const fn new(name: String, tx: mpsc::Sender<ChargerMspcType>) -> Self {
        Self {
            _name: name,
            tx,
            transaction_id: None,
        }
    }
}
struct Context {
    chargers: Arc<Mutex<HashMap<String, Charger>>>,
}

impl Context {
    pub async fn add_charger(&self, name: String, tx: mpsc::Sender<ChargerMspcType>) {
        self.chargers
            .lock()
            .await
            .insert(name.clone(), Charger::new(name, tx));
    }

    pub async fn del_charger(&self, name: &str) {
        self.chargers.lock().await.remove(name);
    }

    pub async fn get_charger(&self, name: &str) -> Option<Charger> {
        let chargers = self.chargers.lock().await;
        if let Some(charger) = chargers.get(name) {
            return Some(charger.clone());
        }
        drop(chargers);
        None
    }

    pub async fn start_transaction(&self, name: &str, transaction_id: i32) {
        let mut chargers = self.chargers.lock().await;
        if let Some(charger) = chargers.get_mut(name) {
            charger.transaction_id = Some(transaction_id);
        }
        drop(chargers);
    }

    pub async fn stop_transaction(&self, name: &str) -> Option<i32> {
        let mut chargers = self.chargers.lock().await;
        if let Some(charger) = chargers.get_mut(name) {
            let id = charger.transaction_id;
            charger.transaction_id = None;
            return id;
        }
        drop(chargers);
        None
    }
}

pub(crate) fn get_context() -> Arc<Context> {
    CONTEXT.get().expect("CONTEXT not set").clone()
}

pub fn init() {
    CONTEXT.get_or_init(|| {
        Arc::new(Context {
            chargers: Arc::new(Mutex::new(HashMap::new())),
        })
    });
}

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
    let ocpp_app = Router::new().route("/:station_id", get(ws_connect)).layer(
        TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true)),
    );

    // build our control app with some routes
    let control_app = Router::new()
        .route("/status", get(get_status).post(post_status))
        .route("/start", get(get_404).post(post_start))
        .route("/stop", get(get_404).post(post_stop));

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

fn validate_station(station: &str) -> Result<(), ErrorUnknownStationId> {
    if station == "wallbox" {
        Ok(())
    } else {
        Err(ErrorUnknownStationId {})
    }
}

struct ErrorUnknownStationId {}

impl IntoResponse for ErrorUnknownStationId {
    fn into_response(self) -> Response {
        Response::default()
    }
}

async fn ws_connect(Path(station): Path<String>, ws: WebSocketUpgrade) -> impl IntoResponse {
    tracing::info!("Incoming connection from station {}", station);
    let name = station.clone();
    let (tx, rx) = mpsc::channel(1);
    get_context().add_charger(name.clone(), tx).await;

    match validate_station(&station) {
        Ok(()) => ws.on_upgrade(|socket| handle_socket(socket, name, rx)),
        Err(_) => ws.on_upgrade(handle_error),
    }
}

async fn get_404() {}
async fn get_status() {}
async fn post_status() {}

async fn post_start() {
    let charger_name = "wallbox";
    if let Some(charger) = get_context().get_charger(charger_name).await {
        let request = rpc::enums::OcppPayload::RemoteStartTransaction(
            RemoteStartTransactionKind::Request(RemoteStartTransactionRequest {
                connector_id: None,
                id_tag: "random".to_string(),
                charging_profile: None,
            }),
        );

        let request = OcppCall {
            message_type_id: 2,
            message_id: "RANDOM STRING MESSAGE ID".to_string(),
            action: rpc::enums::OcppActionEnum::RemoteStartTransaction,
            payload: request,
        };
        let _ = charger.tx.send(request).await;
    }
}
async fn post_stop() {
    let charger_name = "wallbox";
    if let Some(charger) = get_context().get_charger(charger_name).await {
        if let Some(transaction_id) = get_context().stop_transaction(charger_name).await {
            let request = rpc::enums::OcppPayload::RemoteStopTransaction(
                RemoteStopTransactionKind::Request(RemoteStopTransactionRequest { transaction_id }),
            );

            let request = OcppCall {
                message_type_id: 2,
                message_id: "RANDOM STRING MESSAGE ID".to_string(),
                action: rpc::enums::OcppActionEnum::RemoteStopTransaction,
                payload: request,
            };
            let _ = charger.tx.send(request).await;
        }
    }
}

async fn handle_socket(socket: WebSocket, name: String, mut rx: mpsc::Receiver<ChargerMspcType>) {
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

async fn handle_error(mut socket: WebSocket) {
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
