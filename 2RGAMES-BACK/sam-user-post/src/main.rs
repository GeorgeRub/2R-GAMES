use http_service::{ResponseMessageBody, http_service_response};
use lambda_http::Body::Text;
use lambda_http::http::StatusCode;
use lambda_http::{Body, Request, Response, run};
use lambda_runtime::{Error, service_fn};
use models::services::user_service::{UserService, UserServiceTrait};
use models::user::User;
use uuid::Uuid;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let body = event.body();
    if body.is_empty() {
        return Ok(http_service_response(
            ResponseMessageBody {
                message: "Body is empty.".to_string(),
            },
            StatusCode::BAD_REQUEST,
        ));
    }
    match body {
        Text(text) => match serde_json::from_str::<User>(text) {
            Ok(mut user) => {
                // let is_user_exist = User::is_user_exists(user.user_email.as_str()).await;
                let is_user_exist = UserService::is_user_exists(&user.user_email.as_str()).await;
                if is_user_exist {
                    println!("User already exists");
                    let message = format!("We have a user with email: {:?}", user.user_email);
                    return Ok(http_service_response(
                        ResponseMessageBody { message },
                        StatusCode::CONFLICT,
                    ));
                }

                user.user_id = Some(Uuid::now_v7().to_string());
                user.user_active = Some(true);
                match UserService::save(&user).await {
                    Ok(saved_user) => Ok(http_service_response(saved_user, StatusCode::OK)),
                    Err(e) => {
                        println!("Error saving user {:?}", e);
                        Ok(http_service_response(
                            ResponseMessageBody {
                                message: format!("{}", e),
                            },
                            StatusCode::OK,
                        ))
                    }
                }
            }
            Err(e) => {
                println!("Error parsing body: {:?}", e);
                Ok(http_service_response(
                    ResponseMessageBody {
                        message: "Server error for parsing body. \r Required parameters are user_email, user_first_name, user_last_name".to_string(),
                    },
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            }
        },
        _ => {
            println!("User post body is empty");
            Ok(http_service_response(
                ResponseMessageBody {
                    message: "Wrong type of body.".to_string(),
                },
                StatusCode::BAD_REQUEST,
            ))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
