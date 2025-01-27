use std::{error::Error, path::Path, sync::mpsc};

use notify::{recommended_watcher, EventKind, Watcher};

fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = mpsc::channel();

    let mut watcher = recommended_watcher(tx)?;

    watcher.watch(Path::new("."), notify::RecursiveMode::Recursive)?;

    for res in rx {
        println!("Evoked event: {:?}", res);
        match res {
            Ok(event) if matches!(event.kind, EventKind::Create(_)) => {
                println!("{:?}", event.paths)
            }
            Err(e) => {
                println!("{:?}", e);
                break;
            }
            _ => {}
        };
    }

    Ok(())
}
