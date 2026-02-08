use service::HelloServiceClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transport = tarpc::serde_transport::tcp::connect(
        "127.0.0.1:8080",
        tarpc::tokio_serde::formats::Bincode::default,
    )
    .await?;

    let client = HelloServiceClient::new(tarpc::client::Config::default(), transport).spawn();

    let _ = dbg!(
        client
            .hello(tarpc::context::current(), "Jack".to_string())
            .await?
    );

    Ok(())
}
