use axum::extract::ws::Message;

use crate::authorization::authorize::handle_authorize;
use crate::handlers::workflows::{
    handle_heartbeat, handle_meter_values, handle_remote_start_transaction,
    handle_remote_stop_transaction, handle_start_transaction, handle_status_notification,
    handle_stop_transaction,
};
use crate::provisioning::bootnotification::handle_bootnotification;
use crate::rpc::enums::OcppPayload;
use crate::rpc::messages::{OcppCall, OcppCallResult};
use crate::{handlers::error::handle_error, rpc::messages::OcppMessageType};

pub async fn parse(msg: Message, charger_name: &str) -> Result<Option<Message>, ()> {
    // Skip any non-Text messages...

    // serialize or die
    println!("got raw message {msg:?}");

    let msg_text = match msg.into_text() {
        Ok(msg) => msg,
        Err(err) => {
            tracing::error!("error {err:?}");
            return Err(());
        }
    };
    let Ok(ocpp_message_type) = serde_json::from_str(msg_text.as_str()) else {
        handle_error(Message::Text("failed to parse call".to_string())).await;
        return Err(());
    };

    parse_ocpp_message_type(&ocpp_message_type).await;
    let message = match OcppCall::try_from(ocpp_message_type.clone()) {
        Ok(message) => message,
        Err(err) => {
            tracing::error!("error {err:?}");
            return Err(());
        }
    };

    println!("got message {message:?}");
    let response = match message.payload {
        OcppPayload::Authorize(authorize_kind) => handle_authorize(authorize_kind).await,
        OcppPayload::BootNotification(boot_notification_kind) => {
            handle_bootnotification(boot_notification_kind).await
        }
        OcppPayload::Heartbeat(heartbeat_kind) => handle_heartbeat(heartbeat_kind).await,
        OcppPayload::MeterValues(metervalues_kind) => {
            handle_meter_values(metervalues_kind, charger_name).await
        }
        /*OcppPayload::ChangeAvailability(_) => todo!(),
        OcppPayload::DataTransfer(_) => todo!(),
        OcppPayload::GetChargingProfile(_) => todo!(),
        OcppPayload::GetLog(_) => todo!(),
        OcppPayload::GetMonitoringReport(_) => todo!(),
        OcppPayload::GetReport(_) => todo!(),
        OcppPayload::GetVariables(_) => todo!(),
        OcppPayload::LogStatusNotification(_) => todo!(),
        OcppPayload::NotifyChargingLimit(_) => todo!(),
        OcppPayload::NotifyCustomerInformation(_) => todo!(),
        OcppPayload::NotifyDisplayMessages(_) => todo!(),
        OcppPayload::NotifyEVChargingNeeds(_) => todo!(),
        OcppPayload::NotifyEVChargingSchedule(_) => todo!(),
        OcppPayload::NotifyEvent(_) => todo!(),
        OcppPayload::NotifyMonitoringReport(_) => todo!(),
        OcppPayload::NotifyReport(_) => todo!(),
        OcppPayload::ReportChargingProfiles(_) => todo!(),*/
        OcppPayload::RemoteStartTransaction(remote_start_transaction_kind) => {
            handle_remote_start_transaction(remote_start_transaction_kind).await
        }
        OcppPayload::RemoteStopTransaction(remote_stop_transaction_kind) => {
            handle_remote_stop_transaction(remote_stop_transaction_kind).await
        }
        /*OcppPayload::Reset(_) => todo!(),
        OcppPayload::SecurityEventNotification(_) => todo!(),
        OcppPayload::SendLocalList(_) => todo!(),
        OcppPayload::SetChargingProfile(_) => todo!(),
        OcppPayload::SetDisplayMessage(_) => todo!(),
        OcppPayload::SetMonitoringBase(_) => todo!(),
        OcppPayload::SetMonitoringLevel(_) => todo!(),
        OcppPayload::SetNetworkProfile(_) => todo!(),
        OcppPayload::SetVariableMonitoring(_) => todo!(),
        OcppPayload::SetVariables(_) => todo!(), */
        OcppPayload::StartTransaction(start_transaction_kind) => {
            handle_start_transaction(start_transaction_kind, charger_name).await
        }
        OcppPayload::StopTransaction(stop_transaction_kind) => {
            handle_stop_transaction(stop_transaction_kind).await
        }
        OcppPayload::StatusNotification(status_notification_kind) => {
            handle_status_notification(status_notification_kind).await
        }
        /* OcppPayload::TransactionEvent(_) => todo!(),
        OcppPayload::TriggerMessage(_) => todo!(),
        OcppPayload::UnlockConnector(_) => todo!(),*/
        _ => {
            tracing::error!("handlers/messages.rs match message.payload has no match");
            return Err(());
        }
    };

    if let Some(response) = response {
        let response = OcppCallResult {
            message_type_id: 3,
            message_id: message.message_id,
            payload: response,
        };
        println!("response: {response:#?}");
        let response_string = match serde_json::to_string(&response) {
            Ok(response) => response,
            Err(err) => {
                tracing::error!("error {err:?}");
                return Err(());
            }
        };
        return Ok(Some(Message::Text(response_string)));
    }
    Ok(None)
}

async fn parse_ocpp_message_type(ocpp_message: &OcppMessageType) {
    match ocpp_message {
        // Call: [<MessageTypeId>, "<MessageId>", "<Action>", {<Payload>}]
        OcppMessageType::Call(message_type_id, _message_id, _action, _payload) => {
            // Validate message type id is 2 for Call
            if message_type_id.ne(&2) {
                handle_error(Message::Text("Wrong message type id".into())).await;
            }
            //handle_message(message_id, action /*, payload*/).await;
        }

        // CallResult: [<MessageTypeId>, "<MessageId>", {<Payload>}]
        OcppMessageType::CallResult(message_type_id, _message_id, _payload) => {
            // Validate message type id is 3 for CallResult
            if message_type_id.ne(&3) {
                handle_error(Message::Text("Wrong message type id".into())).await;
            }
        }

        // CallError: [<MessageTypeId>, "<MessageId>", "<errorCode>", "<errorDescription>", {<errorDetails>}]
        OcppMessageType::CallError(
            message_type_id,
            _message_id,
            _error_code,
            _error_description,
            _error_details,
        ) => {
            // Validate message type id is 4 for CallError
            if message_type_id.ne(&4) {
                handle_error(Message::Text("Wrong message type id".into())).await;
            }
        }
    }
}
