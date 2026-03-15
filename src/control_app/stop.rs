use std::sync::Arc;

use crate::{
    context::Context,
    rpc::{self, enums::RemoteStopTransactionKind, messages::OcppCall},
};
use axum::{Json, extract::State};
use rust_ocpp::v1_6::messages::remote_stop_transaction::RemoteStopTransactionRequest;
use serde::Deserialize;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct StopInput {
    connector_id: u32,
}

pub async fn post_stop(State(context): State<Arc<Mutex<Context>>>, Json(input): Json<StopInput>) {
    tracing::info!("post stop {input:#?}");
    let charger_name = "wallbox";
    let transaction_id = context
        .lock()
        .await
        .stop_transaction(input.connector_id, charger_name);
    let context_lock = context.lock().await;
    if let Some(charger) = context_lock.get_charger(charger_name) {
        if let Some(transaction_id) = transaction_id {
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
    drop(context_lock);
}
