use std::{error::Error, io::Write};

use rust_tonic::chat::{chatbot_client::ChatbotClient, ChatMessage};
use std::io;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut client = ChatbotClient::connect("http://127.0.0.1:3000").await?;
    let (tx, rx) = mpsc::channel(100);

    tokio::spawn(async move {
        let mut response_stream = client
            .converse(Request::new(ReceiverStream::new(rx)))
            .await
            .unwrap()
            .into_inner();
        while let Some(message) = response_stream.message().await.unwrap() {
            println!("<<< {}", message.content);
        }
    });

    println!("-------Chat by typing a message and type \"exit\" to quit-------");
    loop {
        let mut message = "".to_string();

        io::stdout().flush()?;
        io::stdin().read_line(&mut message)?;

        message = message.trim().to_string();

        if message == "exit".to_string() {
            break;
        } else {
            tx.send(ChatMessage { content: message }).await?;
        }
    }

    Ok(())
}
