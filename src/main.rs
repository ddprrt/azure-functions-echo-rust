use std::{env, error::Error, net::SocketAddr};

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router, Server};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new().route("/api/:endpoint", post(handler));

    let port = match env::var("FUNCTIONS_CUSTOMHANDLER_PORT") {
        Ok(val) => val.parse().expect("Custom Handler port is not a number"),
        Err(_) => 3000,
    };
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    Server::bind(&addr).serve(app.into_make_service()).await?;

    Ok(())
}

async fn handler(Json(payload): Json<Value>) -> impl IntoResponse {
    (StatusCode::OK, Json(payload))
}
