// use tracing_subscriber::filter::{EnvFilter, LevelFilter};

// use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::{Deserialize, Serialize};
use tracing::{info, warn, Level};
use tracing_subscriber::FmtSubscriber;
///This is a simple multiplier function that return the product
///returns a JSON object with the product of the two numbers
///Takes an event object with two numbers: x, y
#[derive(Deserialize)]
struct Request {
    x: i32,
    y: i32,
}

#[derive(Serialize)]
struct Response {
    product: i32,
}


/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful information from the event
    let x = event.payload.x;
    let y = event.payload.y;
    
    // Multiply
    let product = x * y;
    
    let resp = Response {
        product
    };
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialze logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber).expect("Unable to set glocal default");
    
    info!("Starting the redactr service");
    warn!("This is a warning");

    // tracing_subscriber::fmt()
    //     .with_env_filter(
    //         EnvFilter::builder()
    //             .with_default_directive(LevelFilter::INFO.into())
    //             .from_env_lossy(),
    //     )
    //     // disable printing the name of the module in every log line.
    //     .with_target(false)
    //     // disabling time is handy because CloudWatch will add the ingestion time.
    //     .without_time()
    //     .init();

    run(service_fn(function_handler)).await
}
