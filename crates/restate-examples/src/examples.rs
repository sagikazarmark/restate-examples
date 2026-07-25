use std::collections::HashMap;
use std::convert::Infallible;

use restate_sdk::prelude::*;

pub(crate) struct Counter;

const COUNT: &str = "count";

#[restate_sdk::object]
impl Counter {
    #[handler]
    async fn get(&self, ctx: SharedObjectContext<'_>) -> Result<u64, TerminalError> {
        Ok(ctx.get::<u64>(COUNT).await?.unwrap_or(0))
    }

    #[handler]
    async fn add(&self, ctx: ObjectContext<'_>, val: u64) -> Result<u64, TerminalError> {
        let current = ctx.get::<u64>(COUNT).await?.unwrap_or(0);
        let new = current + val;
        ctx.set(COUNT, new);
        Ok(new)
    }

    #[handler]
    async fn increment(&self, ctx: ObjectContext<'_>) -> Result<u64, TerminalError> {
        self.add(ctx, 1).await
    }

    #[handler]
    async fn reset(&self, ctx: ObjectContext<'_>) -> Result<(), TerminalError> {
        ctx.clear(COUNT);
        Ok(())
    }
}

pub(crate) struct FailureExample;

#[derive(Debug, thiserror::Error)]
#[error("I'm very bad, retry me")]
pub(crate) struct MyError;

#[restate_sdk::service]
impl FailureExample {
    #[handler(name = "doRun")]
    async fn do_run(&self, context: Context<'_>) -> Result<(), TerminalError> {
        context
            .run::<_, _, ()>(|| async move {
                if rand::random::<u32>().is_multiple_of(4) {
                    Err(TerminalError::new("Failed!!!"))?
                }

                Err(MyError)?
            })
            .await?;

        Ok(())
    }
}

pub(crate) struct Greeter;

#[restate_sdk::service]
impl Greeter {
    #[handler]
    async fn greet(&self, _: Context<'_>, name: String) -> Result<String, Infallible> {
        Ok(format!("Greetings {name}"))
    }
}

pub(crate) struct RunExample(pub(crate) reqwest::Client);

#[restate_sdk::service]
impl RunExample {
    #[handler(name = "doRun")]
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

#[cfg(test)]
mod tests {
    use restate_sdk::service::Discoverable;

    use super::{Counter, FailureExample, Greeter, RunExample};

    #[test]
    fn discovers_existing_service_api() {
        let counter = <Counter as Discoverable>::discover();
        assert_eq!(counter.name.as_str(), "Counter");

        let failure_example = <FailureExample as Discoverable>::discover();
        assert_eq!(failure_example.name.as_str(), "FailureExample");
        assert!(
            failure_example
                .handlers
                .iter()
                .any(|handler| handler.name.as_str() == "doRun")
        );

        let greeter = <Greeter as Discoverable>::discover();
        assert_eq!(greeter.name.as_str(), "Greeter");

        let run_example = <RunExample as Discoverable>::discover();
        assert_eq!(run_example.name.as_str(), "RunExample");
        assert!(
            run_example
                .handlers
                .iter()
                .any(|handler| handler.name.as_str() == "doRun")
        );
    }
}
