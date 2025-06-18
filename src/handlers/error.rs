use axum::extract::ws::Message;
use tracing::{error, info};

pub async fn handle_error(error: Message) {
    info!("Entered error_handler");
    error!("Error message: {error:?}");
}
