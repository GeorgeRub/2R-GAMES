use http_service::{ResponseMessageBody, http_service_response};
use lambda_http::http::StatusCode;
use lambda_http::{Body, Request, RequestExt, Response, run};
use lambda_runtime::{Error, service_fn};
use models::services::user_service::{UserService, UserServiceTrait};
use serde_json::json;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let ev = event;
    let queries = ev.query_string_parameters();
    let users = UserService::get_all_users(queries).await;
    println!("USERS {:?}", users);
    if users.len() > 0 {
        return Ok(http_service_response(users.clone(), StatusCode::OK));
    }
    Ok( http_service_response(json!("{}"), StatusCode::OK))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
