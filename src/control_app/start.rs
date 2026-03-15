use std::sync::Arc;

use crate::{
    context::Context,
    rpc::{self, enums::RemoteStartTransactionKind, messages::OcppCall},
};
use axum::{extract::State, Json};
use rust_ocpp::v1_6::messages::remote_start_transaction::RemoteStartTransactionRequest;
use serde::Deserialize;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct StartInput {
    connector_id: Option<u32>,
}

pub async fn post_start(State(context): State<Arc<Mutex<Context>>>, Json(input): Json<StartInput>) {
    tracing::debug!("post start {input:#?}");
    let charger_name = "wallbox";
    let context_lock = context.lock().await;
    if let Some(charger) = context_lock.get_charger(charger_name) {
        let request = rpc::enums::OcppPayload::RemoteStartTransaction(
            RemoteStartTransactionKind::Request(RemoteStartTransactionRequest {
                connector_id: input.connector_id,
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
