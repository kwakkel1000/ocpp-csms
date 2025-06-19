use crate::{
    context::get_context,
    rpc::{self, enums::SetChargingProfileKind, messages::OcppCall},
};
use axum::Json;
use chrono::{DateTime, Utc};
use rand::Rng;
use rust_decimal::Decimal;
use rust_ocpp::v1_6::{
    messages::set_charging_profile::SetChargingProfileRequest,
    types::{
        ChargingProfile, ChargingProfileKindType, ChargingProfilePurposeType, ChargingRateUnitType,
        ChargingSchedule, ChargingSchedulePeriod,
    },
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Input {
    limit: i64,
    phases: Option<i32>,
    min_charging_rate: Option<i64>,
}

pub async fn post_profile(Json(input): Json<Input>) {
    let charger_name = "wallbox";
    if let Some(charger) = get_context().get_charger(charger_name).await {
        let transaction_id = charger.transaction_id;
        let limit = Decimal::new(input.limit, 1);
        let min_charging_rate = input.min_charging_rate.map(|min| Decimal::new(min, 1));
        let now: DateTime<Utc> = Utc::now();
        let charging_schedule = ChargingSchedule {
            duration: None,
            start_schedule: Some(now),
            charging_rate_unit: ChargingRateUnitType::A,
            charging_schedule_period: vec![ChargingSchedulePeriod {
                start_period: 0,
                limit,
                // Wallbox ignores this
                number_phases: input.phases,
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
}
