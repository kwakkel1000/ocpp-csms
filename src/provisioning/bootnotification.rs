//use crate::handlers::response::handle_response;
use crate::rpc::enums::BootNotificationKind;
use crate::rpc::enums::OcppPayload;
use chrono::DateTime;
use chrono::Utc;
use rust_ocpp::v1_6::messages::boot_notification::BootNotificationResponse;
use rust_ocpp::v1_6::types::RegistrationStatus;

pub async fn handle_bootnotification(request: BootNotificationKind) -> Option<OcppPayload> {
    // check if its a request or response
    match request {
        BootNotificationKind::Request(req) => {
            println!("{req:#?}");
            let now: DateTime<Utc> = Utc::now();
            let response = BootNotificationResponse {
                current_time: now,
                interval: 10,
                status: RegistrationStatus::Accepted,
            };
            Some(OcppPayload::BootNotification(
                BootNotificationKind::Response(response),
            ))
            //handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await
        }
        BootNotificationKind::Response(_) => {
            None
            //handle_response(Message::Text("Got response".into())).await
        }
    }
}
