use std::{error::Error, fs, path::Path, time::Duration};

use tokio::sync::mpsc::{self, Sender};

async fn track_files(tx: Sender<String>) -> anyhow::Result<()> {
    let path = Path::new("test");
    let mut value = fs::read_to_string(path)?;

    loop {
        let new_value = fs::read_to_string(path)?;
        if value != new_value {
            value = new_value;
            tx.send(value.clone()).await?;
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (tx, mut rx) = mpsc::channel(1);

    let (a, b) = tokio::join!(
        track_files(tx),
        tokio::spawn(async move {
            while let Some(res) = rx.recv().await {
                println!("Value changed, new value: {}", res);
            }
        })
    );

    if let Err(e) = a {
        println!("{:?}", e);
    }

    if let Err(e) = b {
        println!("{:?}", e);
    }

    Ok(())
}
