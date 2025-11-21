use lambda_http::http::StatusCode;
use lambda_http::{Body, Response};
use serde::Serialize;
use serde_json::json;

fn cors_headers() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Access-Control-Allow-Origin", "http://localhost:4200"),
        ("Access-Control-Allow-Methods", "GET, POST, OPTIONS"),
        ("Access-Control-Allow-Headers", "Content-Type"),
    ]
}

pub fn http_service_response<T: Serialize>(body: T, status: StatusCode) -> Response<Body> {
  let mut response =  Response::builder()
        .status(status)
        .header("content-type", "application/json")
        .body(json!(body).to_string().into())
        .map_err(Box::new)
        .unwrap();

    for (k, v) in cors_headers() {
        response.headers_mut().insert(
            k,
            v.parse().unwrap(),
        );
    }

    response
}

#[derive(Serialize)]
pub struct ResponseMessageBody {
    pub message: String,
}
