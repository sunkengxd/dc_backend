use crate::{message::Message, validated_json::ValidatedJson};
use axum::{
    Json, Router,
    http::{StatusCode, header},
    response::{Html, IntoResponse},
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
    let html = tokio::fs::read_to_string("./src/static/index.html")
        .await
        .unwrap_or_else(|_| "<h1>index.html not found</h1>".to_string());
    Html(html)
}

async fn script() -> impl IntoResponse {
    match tokio::fs::read_to_string("./src/static/script.js").await {
        Ok(content) => ([(header::CONTENT_TYPE, "text/javascript")], content).into_response(),

        Err(_) => (StatusCode::NOT_FOUND, "No script file".to_string()).into_response(),
    }
}

async fn styles() -> impl IntoResponse {
    match tokio::fs::read_to_string("./src/static/styles.css").await {
        Ok(content) => ([(header::CONTENT_TYPE, "text/css")], content).into_response(),

        Err(_) => (StatusCode::NOT_FOUND, "No styles CSS".to_string()).into_response(),
    }
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(index))
        .route("/control", post(receive_message))
        .route("/styles.css", get(styles))
        .route("/script.js", get(script))
}
