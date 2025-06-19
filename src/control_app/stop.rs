use crate::{
    context::get_context,
    rpc::{self, enums::RemoteStopTransactionKind, messages::OcppCall},
};
use rust_ocpp::v1_6::messages::remote_stop_transaction::RemoteStopTransactionRequest;

pub async fn post_stop() {
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
