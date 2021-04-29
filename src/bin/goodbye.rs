use lambda_http::{handler, lambda, Context, IntoResponse, Request};
use serde_json::json;

type Error = Box<dyn std::error::Error + Sync + Send + 'static>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(goodbye)).await?;
    Ok(())
}

async fn goodbye(_: Request, _: Context) -> Result<impl IntoResponse, Error> {
    Ok(json!({
        "message": "Goodbye Serverless and Rust 1.45.2 🌛"
    }))
}
