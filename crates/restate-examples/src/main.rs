use anyhow::Result;
use restate_sdk::{endpoint::Endpoint, http_server::HttpServer};

mod examples;
use examples::*;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    HttpServer::new(
        Endpoint::builder()
            .bind(CounterImpl.serve())
            .bind(FailureExampleImpl.serve())
            .bind(GreeterImpl.serve())
            .bind(RunExampleImpl(reqwest::Client::new()).serve())
            .build(),
    )
    .listen_and_serve("0.0.0.0:9080".parse().unwrap())
    .await;

    Ok(())
}
