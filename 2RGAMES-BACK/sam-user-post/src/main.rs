use lambda_http::Body::Text;
use lambda_http::{Body, Request, Response, run};
use lambda_runtime::{Error, service_fn};
use models::user::{User};
use serde_json::json;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    println!("User post event {:?}", event);

    let body = event.body();
    println!("User post body {:?}", body);

    if body.is_empty() {
        return Err(Error::from("Empty body!"));
    }

    // let query = event.query_string_parameters();
    // println!("User post query {:?}", query);

    match body {
        Text(text) => {
            match serde_json::from_str::<User>(text) {
                Ok(user) => println!("User post {:?}", user),
                Err(e) => {
                    println!("Error {:?}", e);
                    return Err(Error::from(e));
                }
            }
            println!("User post text {:?}", text)
        }
        _ => println!("User post body is empty"),
    }

    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(json!("{}").to_string().into())
        .map_err(Box::new)?;
    println!("RESPONSE {:?}", resp);
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
