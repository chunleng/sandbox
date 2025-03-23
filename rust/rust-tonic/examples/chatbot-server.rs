use std::{error::Error, thread::sleep, time::Duration};

use rust_tonic::chat::{
    chatbot_server::{Chatbot, ChatbotServer},
    ChatMessage,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status, Streaming};

#[derive(Default)]
struct AffectionateChatbot {}

#[tonic::async_trait]
impl Chatbot for AffectionateChatbot {
    type ConverseStream = ReceiverStream<Result<ChatMessage, Status>>;

    async fn converse(
        &self,
        req: Request<Streaming<ChatMessage>>,
    ) -> Result<Response<Self::ConverseStream>, Status> {
        let mut stream = req.into_inner();
        let (tx, rx) = mpsc::channel(100);
        tokio::spawn(async move {
            while let Some(message) = stream.message().await.unwrap() {
                println!("Received {:?}", message);
                sleep(Duration::from_secs(2));
                let _ = tx
                    .send(Ok(ChatMessage {
                        content: format!("{}? I love it!", message.content),
                    }))
                    .await;
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Server::builder()
        .add_service(ChatbotServer::new(AffectionateChatbot::default()))
        .serve("127.0.0.1:3000".parse().unwrap())
        .await?;
    Ok(())
}
