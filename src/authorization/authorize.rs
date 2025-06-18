//use crate::handlers::response::handle_response;
use crate::rpc::enums::{AuthorizeKind, OcppPayload};
//use axum::extract::ws::Message;
use rust_ocpp::v1_6::{
    messages::authorize::AuthorizeResponse,
    types::{AuthorizationStatus, IdTagInfo},
};

pub async fn handle_authorize(request: AuthorizeKind) -> Option<OcppPayload> {
    // check if its a request or response
    match request {
        AuthorizeKind::Request(req) => {
            println!("{req:#?}");
            let response = AuthorizeResponse {
                id_tag_info: IdTagInfo {
                    expiry_date: None,
                    parent_id_tag: None,
                    status: AuthorizationStatus::Accepted,
                },
            };
            Some(OcppPayload::Authorize(AuthorizeKind::Response(response)))
            //handle_response(Message::Text(serde_json::to_string(&req).unwrap())).await
        }
        AuthorizeKind::Response(_) => {
            None
            //handle_response(Message::Text("Got response".into())).await
        }
    }
}
