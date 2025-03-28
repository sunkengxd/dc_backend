use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                // TODO: properly format this message
                let message = format!(
                    r#"
                    {{
                        "message": "Input validation error"
                        "errors": [{self}]
                    }}"#
                )
                .replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumJsonRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}
