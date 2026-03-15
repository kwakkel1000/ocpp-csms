use std::{collections::HashMap, sync::Arc};

use crate::{
    connector::Connector,
    context::Context,
    rpc::{
        self,
        enums::{GetCompositeScheduleKind, GetConfigurationKind},
        messages::OcppCall,
    },
};
use axum::{Json, extract::State, response::IntoResponse};
use rand::RngExt;
use rust_ocpp::v1_6::messages::get_composite_schedule::GetCompositeScheduleRequest;
use rust_ocpp::v1_6::messages::get_configuration::GetConfigurationRequest;
use serde::Serialize;
use tokio::sync::Mutex;

#[derive(Serialize, Debug)]
pub struct ResponseConnector {
    pub charging: bool,
    pub current_offered: f64,
    pub charging_current: Option<f64>,
}

#[derive(Serialize, Debug)]
pub struct StatusResponse {
    pub connectors: HashMap<u32, ResponseConnector>,
}

pub async fn get_status(State(context): State<Arc<Mutex<Context>>>) -> impl IntoResponse {
    tracing::debug!("get status");
    let charger_name = "wallbox";
    let context_lock = context.lock().await;
    let mut response = StatusResponse {
        connectors: HashMap::new(),
    };
    if let Some(charger) = context_lock.get_charger(charger_name) {
        tracing::debug!("charger {charger:?}");
        let request = rpc::enums::OcppPayload::GetConfiguration(GetConfigurationKind::Request(
            GetConfigurationRequest { key: None },
        ));

        let message_id: u32 = rand::rng().random();
        let request = OcppCall {
            message_type_id: 2,
            message_id: message_id.to_string(),
            action: rpc::enums::OcppActionEnum::GetConfiguration,
            payload: request,
        };
        let _ = charger.tx.send(request).await;

        let request = rpc::enums::OcppPayload::GetCompositeSchedule(
            GetCompositeScheduleKind::Request(GetCompositeScheduleRequest {
                connector_id: 1,
                duration: 0,
                charging_rate_unit: None,
            }),
        );

        let message_id: u32 = rand::rng().random();
        let request = OcppCall {
            message_type_id: 2,
            message_id: message_id.to_string(),
            action: rpc::enums::OcppActionEnum::GetCompositeSchedule,
            payload: request,
        };
        let _ = charger.tx.send(request).await;
        let connectors = charger.get_connectors();
        for connector_id in connectors {
            if let Some(connector) = charger.get_connector(connector_id) {
                match connector {
                    Connector::Global(_connector_inner) => (),
                    Connector::Real(connector_inner) => {
                        let session = connector_inner.get_session();
                        let current_offered = connector_inner.get_current_offered().unwrap_or(0.0);
                        response.connectors.insert(
                            connector_id,
                            ResponseConnector {
                                charging: session.is_some(),
                                current_offered,
                                charging_current: None,
                            },
                        );
                    }
                }
            }
        }
    } else {
        tracing::debug!("no charger");
    }
    drop(context_lock);
    tracing::debug!("response {response:#?}");

    return Json(response).into_response();
    // (StatusCode::INTERNAL_SERVER_ERROR, Json(())).into_response()
}
