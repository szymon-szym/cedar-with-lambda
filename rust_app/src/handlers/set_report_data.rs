use lambda_http::{run, service_fn, tracing};
use lambda_http::{Body, Error, Request, RequestExt, Response};

#[path ="../authorization/mod.rs"]
pub mod authorization;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}


pub(crate) async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    todo!()
}
