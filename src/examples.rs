use std::collections::HashMap;
use std::convert::Infallible;

use rand::RngCore;
use restate_sdk::prelude::*;

#[restate_sdk::object]
pub(crate) trait Counter {
    #[shared]
    async fn get() -> Result<u64, TerminalError>;
    async fn add(val: u64) -> Result<u64, TerminalError>;
    async fn increment() -> Result<u64, TerminalError>;
    async fn reset() -> Result<(), TerminalError>;
}

pub(crate) struct CounterImpl;

const COUNT: &str = "count";

impl Counter for CounterImpl {
    async fn get(&self, ctx: SharedObjectContext<'_>) -> Result<u64, TerminalError> {
        Ok(ctx.get::<u64>(COUNT).await?.unwrap_or(0))
    }

    async fn add(&self, ctx: ObjectContext<'_>, val: u64) -> Result<u64, TerminalError> {
        let current = ctx.get::<u64>(COUNT).await?.unwrap_or(0);
        let new = current + val;
        ctx.set(COUNT, new);
        Ok(new)
    }

    async fn increment(&self, ctx: ObjectContext<'_>) -> Result<u64, TerminalError> {
        self.add(ctx, 1).await
    }

    async fn reset(&self, ctx: ObjectContext<'_>) -> Result<(), TerminalError> {
        ctx.clear(COUNT);
        Ok(())
    }
}

#[restate_sdk::service]
pub(crate) trait FailureExample {
    #[name = "doRun"]
    async fn do_run() -> Result<(), TerminalError>;
}

pub(crate) struct FailureExampleImpl;

#[derive(Debug, thiserror::Error)]
#[error("I'm very bad, retry me")]
pub(crate) struct MyError;

impl FailureExample for FailureExampleImpl {
    async fn do_run(&self, context: Context<'_>) -> Result<(), TerminalError> {
        context
            .run::<_, _, ()>(|| async move {
                if rand::rng().next_u32() % 4 == 0 {
                    Err(TerminalError::new("Failed!!!"))?
                }

                Err(MyError)?
            })
            .await?;

        Ok(())
    }
}

#[restate_sdk::service]
pub(crate) trait Greeter {
    async fn greet(name: String) -> Result<String, Infallible>;
}

pub(crate) struct GreeterImpl;

impl Greeter for GreeterImpl {
    async fn greet(&self, _: Context<'_>, name: String) -> Result<String, Infallible> {
        Ok(format!("Greetings {name}"))
    }
}

#[restate_sdk::service]
pub(crate) trait RunExample {
    #[name = "doRun"]
    async fn do_run() -> Result<Json<HashMap<String, String>>, HandlerError>;
}

pub(crate) struct RunExampleImpl(pub(crate) reqwest::Client);

impl RunExample for RunExampleImpl {
    async fn do_run(
        &self,
        context: Context<'_>,
    ) -> Result<Json<HashMap<String, String>>, HandlerError> {
        let res = context
            .run(|| async move {
                let req = self.0.get("https://httpbin.org/ip").build()?;

                let res = self
                    .0
                    .execute(req)
                    .await?
                    .json::<HashMap<String, String>>()
                    .await?;

                Ok(Json::from(res))
            })
            .name("get_ip")
            .await?
            .into_inner();

        Ok(res.into())
    }
}
