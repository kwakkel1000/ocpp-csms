use crate::{
    context::get_context,
    rpc::{
        self,
        enums::{GetCompositeScheduleKind, GetConfigurationKind},
        messages::OcppCall,
    },
};
use rand::Rng;
use rust_ocpp::v1_6::messages::get_composite_schedule::GetCompositeScheduleRequest;
use rust_ocpp::v1_6::messages::get_configuration::GetConfigurationRequest;

pub async fn get_status() {
    let charger_name = "wallbox";
    if let Some(charger) = get_context().get_charger(charger_name).await {
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
    }
}
