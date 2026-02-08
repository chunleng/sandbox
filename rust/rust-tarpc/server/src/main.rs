use futures::StreamExt;
use service::HelloService;
use tarpc::server::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut listener = tarpc::serde_transport::tcp::listen(
        "127.0.0.1:8080",
        tarpc::tokio_serde::formats::Bincode::default,
    )
    .await?;

    listener.config_mut().max_frame_length(usize::MAX);

    while let Some(conn) = listener.next().await {
        let conn = match conn {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Connection error: {}", e);
                continue;
            }
        };

        tokio::spawn(async move {
            let server = tarpc::server::BaseChannel::with_defaults(conn);
            server
                .execute(HelloServer.serve())
                .for_each(|response| async move {
                    tokio::spawn(response);
                })
                .await;
        });
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct HelloServer;

impl HelloService for HelloServer {
    async fn hello(self, _context: tarpc::context::Context, name: String) -> String {
        format!("Hello {}", name)
    }
}
