use crate::{message::Message, validated_json::ValidatedJson};
use axum::{Router, routing::post};

async fn receive_message(ValidatedJson(message): ValidatedJson<Message>) -> String {
    format!(
        "Received valid message: speed={}, is_forward={}",
        message.speed, message.is_forward
    )
}

pub fn router() -> Router {
    Router::new().route("/message", post(receive_message))
}
