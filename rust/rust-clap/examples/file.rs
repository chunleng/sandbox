use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(long)]
    password_file: PathBuf,
}

fn main() {
    let args = Cli::try_parse();
    match args {
        Ok(args) => {
            print!("{:?}", fs::read_to_string(args.password_file));
        }
        Err(msg) => {
            let _ = msg.print();
        }
    }
}
