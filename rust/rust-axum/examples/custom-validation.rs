use std::error::Error;

use axum::{Json, Router, debug_handler, response::IntoResponse, routing::post};
use serde::{Deserialize, Deserializer};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new().route("/kids_only", post(kids_only));
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Deserialize)]
struct Kid {
    #[serde(deserialize_with = "validate_age")]
    age: u8,
}

fn validate_age<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    let age = u8::deserialize(deserializer)?;
    if age > 16 {
        return Err(serde::de::Error::custom("You are too old for that"));
    }
    Ok(age)
}

#[debug_handler]
async fn kids_only(kid: Json<Kid>) -> impl IntoResponse {
    return format!("Wonderland! for {} y.o!", kid.age).into_response();
}
