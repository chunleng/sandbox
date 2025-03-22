use std::time::Duration;

use anyhow::{Error, Result};
use axum::{
    Router,
    body::{Body, Bytes},
    extract::Query,
    response::IntoResponse,
    routing::get,
};
use futures_util::{StreamExt, future};
use serde::Deserialize;
use tokio::{
    net::TcpListener,
    sync::mpsc,
    time::{interval, sleep},
};
use tokio_stream::wrappers::{IntervalStream, ReceiverStream};

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/stream_numbers", get(stream_numbers))
        .route("/stream_names", get(stream_names));
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Deserialize)]
struct StreamNumbersRequest {
    end: usize,
}

async fn stream_numbers(
    Query(StreamNumbersRequest { end }): Query<StreamNumbersRequest>,
) -> impl IntoResponse {
    let interval = interval(Duration::from_secs(1));
    let stream = IntervalStream::new(interval)
        .enumerate()
        .take_while(move |(x, _)| future::ready(*x <= end))
        .map(|(x, _)| Ok::<String, Error>(x.to_string()));

    Body::from_stream(stream)
}

async fn stream_names() -> impl IntoResponse {
    let (tx, rx) = mpsc::channel(100);

    _ = tokio::spawn(async move {
        for value in ["John", "Jack", "Jill"] {
            sleep(Duration::from_secs(1)).await;
            if let Err(err) = tx
                .send(Ok::<Bytes, Error>(Bytes::from(format!("{}\n", value))))
                .await
            {
                println!("{:?}", err);
                break;
            }
        }
    });

    Body::from_stream(ReceiverStream::new(rx))
}
