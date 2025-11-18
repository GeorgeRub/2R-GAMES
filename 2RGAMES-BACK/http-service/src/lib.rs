use lambda_http::http::StatusCode;
use lambda_http::{Body, Response};
use serde::Serialize;
use serde_json::json;

pub fn http_service_response<T: Serialize>(body: T, status: StatusCode) -> Response<Body> {
    Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(json!(body).to_string().into())
        .map_err(Box::new)
        .unwrap()
}

#[derive(Serialize)]
pub struct ResponseMessageBody {
    pub message: String,
}
