use anyhow::Result;
use restate_sdk::{endpoint::Endpoint, http_server::HttpServer, service::IntoServiceDefinition};

mod examples;
use examples::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    HttpServer::new(
        Endpoint::builder()
            .bind(Counter.into_service_definition())
            .bind(FailureExample.into_service_definition())
            .bind(Greeter.into_service_definition())
            .bind(RunExample(reqwest::Client::new()).into_service_definition())
            .build(),
    )
    .listen_and_serve("0.0.0.0:9080".parse().unwrap())
    .await;

    Ok(())
}
