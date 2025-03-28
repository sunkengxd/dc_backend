use crate::{message::Message, validated_json::ValidatedJson};
use axum::{
    Json, Router,
    response::Html,
    routing::{get, post},
};
use serde_json::json;

async fn receive_message(
    ValidatedJson(message): ValidatedJson<Message>,
) -> Json<serde_json::Value> {
    let response = json!({
        "message": "Received control",
        "speed": message.speed,
        "is_forward": message.is_forward
    });
    Json(response)
}

async fn index() -> Html<String> {
    // Read the index.html file
    let html = tokio::fs::read_to_string("static/index.html")
        .await
        .unwrap_or_else(|_| "<h1>index.html not found</h1>".to_string());
    Html(html)
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/control", post(receive_message))
}
