use std::{error::Error, fmt};

use tracing::{event, instrument, subscriber::set_global_default, Level};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<(), Box<dyn Error>>  {
    // tracing_subscriber is used to print out the text in stdout
    let subscriber = FmtSubscriber::builder().with_max_level(Level::TRACE).finish();
    set_global_default(subscriber).expect("Fail to set subscriber");

    // Print out log using event!
    event!{Level::TRACE, "Program started!"}

    good(19)?;
    bad()?;

    Ok(())
}

// Use instrument to print out logs with function information
#[instrument(ret)]
fn good(lucky_number: u32) -> Result<String, Box<dyn Error>> {
    event!{Level::INFO, lucky_number = lucky_number}

    // Because we have instrument(ret), return value (apart from Err) will be logged
    Ok(format!("{} is your lucky number", lucky_number))
}

#[instrument(err)]
fn bad()-> Result<(), fmt::Error> {
    // Because we have instrument(err), this error value will be logged
    Err(fmt::Error)
}
