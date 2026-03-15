use std::sync::Arc;

use crate::{
    connector::Connector,
    context::Context,
    rpc::{self, enums::SetChargingProfileKind, messages::OcppCall},
};
use axum::{Json, extract::State};
use chrono::{DateTime, Utc};
use rand::RngExt;
use rust_decimal::Decimal;
use rust_ocpp::v1_6::{
    messages::set_charging_profile::SetChargingProfileRequest,
    types::{
        ChargingProfile, ChargingProfileKindType, ChargingProfilePurposeType, ChargingRateUnitType,
        ChargingSchedule, ChargingSchedulePeriod,
    },
};
use serde::Deserialize;
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ProfileInput {
    connector_id: u32,
    limit: i64,
    phases: Option<i32>,
    min_charging_rate: Option<i64>,
}

#[allow(clippy::cast_precision_loss)]
pub async fn post_profile(
    State(context): State<Arc<Mutex<Context>>>,
    Json(input): Json<ProfileInput>,
) {
    tracing::debug!("post profile {input:#?}");
    let charger_name = "wallbox";
    let ProfileInput {
        connector_id,
        limit,
        phases,
        min_charging_rate,
    } = input;
    let mut context_lock = context.lock().await;
    if let Some(charger) = context_lock.get_charger_mut(charger_name) {
        let mut transaction_id = None;
        if let Some(connector) = charger.get_connector_mut(connector_id) {
            match connector {
                Connector::Real(connector) => {
                    connector.set_current_offered((limit / 10) as f64);
                    transaction_id = connector
                        .get_session()
                        .map(super::super::charger::ChargeSession::get_transaction_id);
                }
                Connector::Global(_connector_inner) => (),
            }
        };
        let limit = Decimal::new(limit, 1);
        let min_charging_rate = min_charging_rate.map(|min| Decimal::new(min, 1));
        let now: DateTime<Utc> = Utc::now();
        let charging_schedule = ChargingSchedule {
            duration: None,
            start_schedule: Some(now),
            charging_rate_unit: ChargingRateUnitType::A,
            charging_schedule_period: vec![ChargingSchedulePeriod {
                start_period: 0,
                limit,
                // Wallbox ignores this
                number_phases: phases,
            }],
            // Wallbox ignores this
            min_charging_rate,
        };
        let cs_charging_profiles = ChargingProfile {
            charging_profile_id: 5,
            transaction_id,
            stack_level: 0,
            charging_profile_purpose: ChargingProfilePurposeType::TxProfile,
            charging_profile_kind: ChargingProfileKindType::Absolute,
            recurrency_kind: None,
            valid_from: None,
            valid_to: None,
            charging_schedule,
        };
        let request = rpc::enums::OcppPayload::SetChargingProfile(SetChargingProfileKind::Request(
            SetChargingProfileRequest {
                connector_id: 1,
                cs_charging_profiles,
            },
        ));

        let message_id: u32 = rand::rng().random();
        let request = OcppCall {
            message_type_id: 2,
            message_id: message_id.to_string(),
            action: rpc::enums::OcppActionEnum::SetChargingProfile,
            payload: request,
        };
        let _ = charger.tx.send(request).await;
    }
    drop(context_lock);
}
