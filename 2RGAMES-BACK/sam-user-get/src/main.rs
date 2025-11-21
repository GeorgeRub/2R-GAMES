use http_service::http_service_response;
use lambda_http::http::StatusCode;
use lambda_http::{Body, Request, Response, run};
use lambda_runtime::{Error, service_fn};
use models::http_communication::pagination::Pagination;
use models::services::user_service::UserService;
use serde_json::json;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let pagination: Pagination = Pagination::new(event);
    let users = UserService::get_all_users(pagination).await;
    println!("Users return {}", users.len());
    Ok(http_service_response(json!(users), StatusCode::OK))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
