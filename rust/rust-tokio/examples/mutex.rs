use std::{error::Error, sync::Arc};

use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mutex = Arc::new(Mutex::new("value".to_string()));

    // If I return await by cloning, the MutexGuard will be removed at the end of the scope
    let value_1 = { mutex.lock().await.clone() };

    {
        println!("value_1: {}", value_1);
        println!("direct call mutex: {}", mutex.lock().await);
    }

    // However, if I return the await itself, the MutexGuard will be returned and therefore the
    // lock will not release.
    let value_2 = { mutex.lock().await };

    {
        println!("value_2: {}", value_2);
        println!(
            "Another call to the mutex cannot be made because value_2 hasn't release the lock"
        );
        println!("{}", mutex.lock().await); // This will never print
    }
    Ok(())
}
