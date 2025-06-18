use axum::extract::ws::Message;
use tracing::log::info;

pub async fn handle_response(response: Message) {
    info!("Entered handle_response");
    info!("response: {response:#?}");
}
