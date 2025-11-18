// use lambda_http::Body::{Text};
use lambda_http::{Body, Request, Response, run};
use lambda_runtime::{Error, service_fn};
use serde_json::json;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let payload = event.body();
    println!("{:?}", payload);
    // let mut user_resp: User = User {
    //     user_email: "".to_string(),
    //     user_id: None,
    // };

    // let exercise = exercise::Exercise::new("Test exercise two".to_string(), "push_up".to_string());
    // let _ = exercise.add_exercise().await;
    // match payload {
    //     Text(text) => {
    //         println!("{:?}", text);
    //         match serde_json::from_str::<User>(text) {
    //             Ok(user) => {
    //                 println!("REQUEST {:?}", user);
    //                 user_resp = user;
    //             }
    //             Err(e) => {
    //                 println!("ERROR {:?}", e);
    //             }
    //         }
    //     }
    //     Binary(bytes) => {
    //         println!("{:?}", bytes);
    //     }
    //     Body::Empty => {
    //         println!("Empty");
    //     }
    //     _ => {}
    // }
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
