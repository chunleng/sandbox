use std::error::Error;

use tokio_postgres::{NoTls, connect};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection_string = "postgresql://root:password@localhost:5432/";
    let (client, connection) = connect(connection_string, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            println!("{:?}", e);
        }
    });

    let row = client.query("SELECT 'connected!'", &[]).await?;
    println!("{}", row[0].get::<usize, &str>(0));
    Ok(())
}
