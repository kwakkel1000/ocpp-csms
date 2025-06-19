use crate::context::get_context;
// use crate::handlers::response::handle_response;
use crate::rpc::enums::{
    GetConfigurationKind, HeartbeatKind, MeterValuesKind, OcppPayload, StartTransactionKind,
    StatusNotificationKind, StopTransactionKind,
};
// use crate::rpc::enums::{
//     HeartbeatKind, MeterValuesKind, OcppPayload, RemoteStartTransactionKind,
//     RemoteStopTransactionKind, StartTransactionKind, StatusNotificationKind, StopTransactionKind,
// };
// use crate::rpc::enums::{
//     CancelReservationKind, ChangeAvailabilityKind, ClearCacheKind, ClearChargingProfileKind,
//     FirmwareStatusNotificationKind, GetCompositeScheduleKind, GetLocalListVersionKind,
//     HeartbeatKind, MeterValuesKind, OcppPayload, RemoteStartTransactionKind,
//     RemoteStopTransactionKind, ReserveNowKind, ResetKind, SendLocalListKind,
//     SetChargingProfileKind, StartTransactionKind, StatusNotificationKind, StopTransactionKind,
//     UnlockConnectorKind, UpdateFirmwareKind,
// };
// use axum::extract::ws::Message;
use chrono::{DateTime, Utc};
use rust_ocpp::v1_6::messages::start_transaction::StartTransactionResponse;
use rust_ocpp::v1_6::messages::status_notification::StatusNotificationResponse;
use rust_ocpp::v1_6::messages::stop_transaction::StopTransactionResponse;
use rust_ocpp::v1_6::messages::{heart_beat::HeartbeatResponse, meter_values::MeterValuesResponse};
use rust_ocpp::v1_6::types::{AuthorizationStatus, IdTagInfo};

// #[allow(unused)]
// pub async fn handle_cancel_reservation(request: CancelReservationKind) {
//     match request {
//         CancelReservationKind::Request(req) => {
//             handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await;
//         }
//         CancelReservationKind::Response(_) => {
//             handle_response(Message::Text("Got response".into())).await;
//         }
//     }
// }
//
// #[allow(unused)]
// pub async fn handle_change_availability(request: ChangeAvailabilityKind) {
//     match request {
//         ChangeAvailabilityKind::Request(req) => {
//             handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await;
//         }
//         ChangeAvailabilityKind::Response(_) => {
//             handle_response(Message::Text("Got response".into())).await;
//         }
//     }
// }
//
// #[allow(unused)]
// pub async fn handle_clear_cache(request: ClearCacheKind) {
//     match request {
//         ClearCacheKind::Request(req) => {
//             handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await;
//         }
//         ClearCacheKind::Response(_) => {
//             handle_response(Message::Text("Got response".into())).await;
//         }
//     }
// }
//
// #[allow(unused)]
// pub async fn handle_clear_charging_profile(request: ClearChargingProfileKind) {
//     match request {
//         ClearChargingProfileKind::Request(req) => {
//             handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await;
//         }
//         ClearChargingProfileKind::Response(_) => {
//             handle_response(Message::Text("Got response".into())).await;
//         }
//     }
// }
//
// #[allow(unused)]
// pub async fn handle_firmware_status_notification(request: FirmwareStatusNotificationKind) {
//     match request {
//         FirmwareStatusNotificationKind::Request(req) => {
//             handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await;
//         }
//         FirmwareStatusNotificationKind::Response(_) => {
//             handle_response(Message::Text("Got response".into())).await;
//         }
//     }
// }
#[allow(unused)]
pub async fn handle_get_configuration(request: GetConfigurationKind) -> Option<OcppPayload> {
    match request {
        GetConfigurationKind::Request(_req) => None,
        GetConfigurationKind::Response(req) => {
            println!("get configuration response: {req:#?}");
            None
        }
    }
}
//
// #[allow(unused)]
// pub async fn handle_get_composite_schedule(request: GetCompositeScheduleKind) {
//     match request {
//         GetCompositeScheduleKind::Request(req) => {
//             handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await;
//         }
//         GetCompositeScheduleKind::Response(_) => {
//             handle_response(Message::Text("Got response".into())).await;
//         }
//     }
// }
//
// #[allow(unused)]
// pub async fn handle_get_local_list_version(request: GetLocalListVersionKind) {
//     match request {
//         GetLocalListVersionKind::Request(req) => {
//             handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await;
//         }
//         GetLocalListVersionKind::Response(_) => {
//             handle_response(Message::Text("Got response".into())).await;
//         }
//     }
// }

pub async fn handle_heartbeat(request: HeartbeatKind) -> Option<OcppPayload> {
    match request {
        HeartbeatKind::Request(req) => {
            println!("heartbeat request: {req:#?}");
            let now: DateTime<Utc> = Utc::now();
            let response = HeartbeatResponse { current_time: now };
            Some(OcppPayload::Heartbeat(HeartbeatKind::Response(response)))
            //handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await;
        }
        HeartbeatKind::Response(_) => {
            None
            //handle_response(Message::Text("Got response".into())).await;
        }
    }
}

pub async fn handle_meter_values(
    request: MeterValuesKind,
    charger_name: &str,
) -> Option<OcppPayload> {
    match request {
        MeterValuesKind::Request(req) => {
            println!("metering values request: {req:#?}");
            if let Some(transaction_id) = req.transaction_id {
                get_context()
                    .start_transaction(charger_name, transaction_id)
                    .await;
            }
            let response = MeterValuesResponse {};
            Some(OcppPayload::MeterValues(MeterValuesKind::Response(
                response,
            )))
            //handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await;
        }
        MeterValuesKind::Response(_) => {
            None
            //handle_response(Message::Text("Got response".into())).await;
        }
    }
}

// pub async fn handle_remote_start_transaction(
//     request: RemoteStartTransactionKind,
// ) -> Option<OcppPayload> {
//     match request {
//         RemoteStartTransactionKind::Request(_req) => None,
//         RemoteStartTransactionKind::Response(req) => {
//             println!("remote start transaction response: {req:#?}");
//             None
//         }
//     }
// }
//
// pub async fn handle_remote_stop_transaction(
//     request: RemoteStopTransactionKind,
// ) -> Option<OcppPayload> {
//     match request {
//         RemoteStopTransactionKind::Request(_req) => None,
//         RemoteStopTransactionKind::Response(req) => {
//             println!("remote stop transaction response: {req:#?}");
//             None
//         }
//     }
// }

// #[allow(unused)]
// pub async fn handle_reserve_now(request: ReserveNowKind) {
//     match request {
//         ReserveNowKind::Request(req) => {
//             handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await;
//         }
//         ReserveNowKind::Response(_) => {
//             handle_response(Message::Text("Got response".into())).await;
//         }
//     }
// }
//
// #[allow(unused)]
// pub async fn handle_reset(request: ResetKind) {
//     match request {
//         ResetKind::Request(req) => {
//             handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await;
//         }
//         ResetKind::Response(_) => {
//             handle_response(Message::Text("Got response".into())).await;
//         }
//     }
// }
//
// #[allow(unused)]
// pub async fn handle_send_local_list(request: SendLocalListKind) {
//     match request {
//         SendLocalListKind::Request(req) => {
//             handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await;
//         }
//         SendLocalListKind::Response(_) => {
//             handle_response(Message::Text("Got response".into())).await;
//         }
//     }
// }
//
// #[allow(unused)]
// pub async fn handle_set_charging_profile(request: SetChargingProfileKind) {
//     match request {
//         SetChargingProfileKind::Request(req) => {
//             handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await;
//         }
//         SetChargingProfileKind::Response(_) => {
//             handle_response(Message::Text("Got response".into())).await;
//         }
//     }
// }

pub async fn handle_start_transaction(
    request: StartTransactionKind,
    charger_name: &str,
) -> Option<OcppPayload> {
    match request {
        StartTransactionKind::Request(req) => {
            println!("start transaction request: {req:#?}");
            let transaction_id = 99;
            get_context()
                .start_transaction(charger_name, transaction_id)
                .await;
            let status = AuthorizationStatus::Accepted;
            let id_tag_info = IdTagInfo {
                expiry_date: None,
                parent_id_tag: None,
                status,
            };
            let response = StartTransactionResponse {
                id_tag_info,
                transaction_id,
            };
            Some(OcppPayload::StartTransaction(
                StartTransactionKind::Response(response),
            ))
        }
        StartTransactionKind::Response(req) => {
            println!("remote start transaction response: {req:#?}");
            None
        }
    }
}

pub async fn handle_stop_transaction(request: StopTransactionKind) -> Option<OcppPayload> {
    match request {
        StopTransactionKind::Request(req) => {
            println!("stop transaction request: {req:#?}");
            let status = AuthorizationStatus::Accepted;
            let id_tag_info = Some(IdTagInfo {
                expiry_date: None,
                parent_id_tag: None,
                status,
            });
            let response = StopTransactionResponse { id_tag_info };
            Some(OcppPayload::StopTransaction(StopTransactionKind::Response(
                response,
            )))
        }
        StopTransactionKind::Response(req) => {
            println!("remote stop transaction response: {req:#?}");
            None
        }
    }
}

pub async fn handle_status_notification(request: StatusNotificationKind) -> Option<OcppPayload> {
    match request {
        StatusNotificationKind::Request(req) => {
            println!("metering values request: {req:#?}");
            let response = StatusNotificationResponse {};
            Some(OcppPayload::StatusNotification(
                StatusNotificationKind::Response(response),
            ))
            //handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await;
        }
        StatusNotificationKind::Response(_) => {
            None
            //handle_response(Message::Text("Got response".into())).await;
        }
    }
}

// #[allow(unused)]
// pub async fn handle_unlock_connector(request: UnlockConnectorKind) {
//     match request {
//         UnlockConnectorKind::Request(req) => {
//             handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await;
//         }
//         UnlockConnectorKind::Response(_) => {
//             handle_response(Message::Text("Got response".into())).await;
//         }
//     }
// }
//
// #[allow(unused)]
// pub async fn handle_update_firmware(request: UpdateFirmwareKind) {
//     match request {
//         UpdateFirmwareKind::Request(req) => {
//             handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await;
//         }
//         UpdateFirmwareKind::Response(_) => {
//             handle_response(Message::Text("Got response".into())).await;
//         }
//     }
// }
