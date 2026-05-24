use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::json;

#[derive(Debug)]
pub struct ApiResponse<T: Serialize> {
    status_code: StatusCode,
    data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            status_code: StatusCode::OK,
            data,
        }
    }

    pub fn created(data: T) -> Self {
        Self {
            status_code: StatusCode::CREATED,
            data,
        }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "status": "success",
            "data": self.data,
            "error": null
        }));
        (self.status_code, body).into_response()
    }
}
