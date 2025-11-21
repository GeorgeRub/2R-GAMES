use http_service::{http_service_response, ResponseMessageBody};
use lambda_http::http::StatusCode;
use lambda_http::Body::Text;
use lambda_http::{run, Body, Request, Response};
use lambda_runtime::{service_fn, Error};
use models::http_communication::user_http_communication::PostUser;
use models::services::user_service::UserService;
use models::user::User;

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
        Text(text) => match serde_json::from_str::<PostUser>(text) {
            Ok(post_user) => {
                let user = User::new(
                    post_user.user_email,
                    post_user.user_first_name,
                    post_user.user_last_name,
                );
                match UserService::save(&user).await {
                    Ok(saved_user) => Ok(http_service_response(
                        ResponseMessageBody {
                            message: format!("User saved: {:?}", &user.user_email),
                        },
                        StatusCode::OK,
                    )),
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
