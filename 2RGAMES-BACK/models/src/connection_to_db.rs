use aws_sdk_dynamodb::Client;

pub async fn get_connection_to_db() -> Client {
    // tracing_subscriber::fmt::init();
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .test_credentials()
        // DynamoDB run locally uses port 8000 by default.
        .endpoint_url("http://192.168.2.58:8000")
        .load()
        .await;
    let dynamodb_local_config = aws_sdk_dynamodb::config::Builder::from(&config).build();
    Client::from_conf(dynamodb_local_config)

}