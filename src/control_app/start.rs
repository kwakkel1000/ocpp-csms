use crate::{
    context::get_context,
    rpc::{self, enums::RemoteStartTransactionKind, messages::OcppCall},
};
use rust_ocpp::v1_6::messages::remote_start_transaction::RemoteStartTransactionRequest;

pub async fn post_start() {
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
