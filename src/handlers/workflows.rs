use std::sync::Arc;

use crate::connector::Connector;
use crate::context::Context;
// use crate::handlers::response::handle_response;
use crate::rpc::enums::{
    GetConfigurationKind, HeartbeatKind, MeterValuesKind, OcppPayload, StartTransactionKind,
    StatusNotificationKind, StopTransactionKind,
};
use axum::extract::State;
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
use rust_ocpp::v1_6::types::{AuthorizationStatus, IdTagInfo, MeterValue, SampledValue};
use tokio::sync::Mutex;

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
            tracing::debug!("get configuration response: {req:#?}");
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
            tracing::debug!("heartbeat request: {req:#?}");
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
    State(context): State<Arc<Mutex<Context>>>,
) -> Option<OcppPayload> {
    match request {
        MeterValuesKind::Request(req) => {
            tracing::debug!("metering values request: {req:#?}");
            let mut context_lock = context.lock().await;
            if let Some(charger) = context_lock.get_charger_mut(charger_name) {
                let connector_id = req.connector_id;
                // let transaction_id = req.transaction_id;
                for meter_value in req.meter_value {
                    let MeterValue {
                        timestamp: _,
                        sampled_value,
                    } = meter_value;
                    for sampled_value in sampled_value {
                        let SampledValue {
                            value,
                            context: _,
                            format: _,
                            measurand,
                            phase: _,
                            location: _,
                            unit: _,
                        } = sampled_value;
                        if let Some(measurand) = measurand {
                            match measurand {
                                // rust_ocpp::v1_6::types::Measurand::CurrentExport => todo!(),
                                // rust_ocpp::v1_6::types::Measurand::CurrentImport => todo!(),
                                rust_ocpp::v1_6::types::Measurand::CurrentOffered => {
                                    if let Ok(value) = value.parse::<f64>() {
                                        if let Some(Connector::Real(connector)) =
                                            charger.get_connector_mut(connector_id)
                                        {
                                            connector.set_current_offered(value);
                                        }
                                    }
                                }
                                // rust_ocpp::v1_6::types::Measurand::EnergyActiveExportRegister => {
                                //     todo!()
                                // }
                                // rust_ocpp::v1_6::types::Measurand::EnergyActiveImportRegister => {
                                //     todo!()
                                // }
                                // rust_ocpp::v1_6::types::Measurand::EnergyReactiveExportRegister => {
                                //     todo!()
                                // }
                                rust_ocpp::v1_6::types::Measurand::EnergyReactiveImportRegister => {
                                    if let Ok(value) = value.parse::<f64>() {
                                        if let Some(connector) =
                                            charger.get_connector_mut(connector_id)
                                        {
                                            connector.set_lifetime_energy_usage(value);
                                        }
                                    }
                                }
                                // rust_ocpp::v1_6::types::Measurand::EnergyActiveExportInterval => {
                                //     todo!()
                                // }
                                // rust_ocpp::v1_6::types::Measurand::EnergyActiveImportInterval => {
                                //     todo!()
                                // }
                                // rust_ocpp::v1_6::types::Measurand::EnergyReactiveExportInterval => {
                                //     todo!()
                                // }
                                // rust_ocpp::v1_6::types::Measurand::EnergyReactiveImportInterval => {
                                //     todo!()
                                // }
                                // rust_ocpp::v1_6::types::Measurand::Frequency => todo!(),
                                // rust_ocpp::v1_6::types::Measurand::PowerActiveExport => todo!(),
                                // rust_ocpp::v1_6::types::Measurand::PowerActiveImport => todo!(),
                                // rust_ocpp::v1_6::types::Measurand::PowerFactor => todo!(),
                                // rust_ocpp::v1_6::types::Measurand::PowerOffered => todo!(),
                                // rust_ocpp::v1_6::types::Measurand::PowerReactiveExport => todo!(),
                                // rust_ocpp::v1_6::types::Measurand::PowerReactiveImport => todo!(),
                                // rust_ocpp::v1_6::types::Measurand::Rpm => todo!(),
                                // rust_ocpp::v1_6::types::Measurand::SoC => todo!(),
                                // rust_ocpp::v1_6::types::Measurand::Temperature => todo!(),
                                // rust_ocpp::v1_6::types::Measurand::Voltage => todo!(),
                                _ => {}
                            }
                        }
                    }
                }
            }
            drop(context_lock);
            if let Some(transaction_id) = req.transaction_id {
                context.lock().await.start_transaction(
                    charger_name,
                    req.connector_id,
                    transaction_id,
                );
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
//             tracing::debug!("remote start transaction response: {req:#?}");
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
//             tracing::debug!("remote stop transaction response: {req:#?}");
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
    transaction_id: i32,
    State(context): State<Arc<Mutex<Context>>>,
) -> Option<OcppPayload> {
    match request {
        StartTransactionKind::Request(req) => {
            tracing::info!("start transaction request: {req:#?}");
            context
                .lock()
                .await
                .start_transaction(charger_name, req.connector_id, transaction_id);
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
            tracing::info!("remote start transaction response: {req:#?}");
            None
        }
    }
}

pub async fn handle_stop_transaction(request: StopTransactionKind) -> Option<OcppPayload> {
    match request {
        StopTransactionKind::Request(req) => {
            tracing::info!("stop transaction request: {req:#?}");
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
            tracing::info!("remote stop transaction response: {req:#?}");
            None
        }
    }
}

pub async fn handle_status_notification(
    request: StatusNotificationKind,
    charger_name: &str,
    State(context): State<Arc<Mutex<Context>>>,
) -> Option<OcppPayload> {
    match request {
        StatusNotificationKind::Request(req) => {
            tracing::debug!("metering values request: {req:#?}");
            let mut context_lock = context.lock().await;
            if let Some(charger) = context_lock.get_charger_mut(charger_name) {
                charger.add_connector(req.connector_id);
            }
            drop(context_lock);
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
