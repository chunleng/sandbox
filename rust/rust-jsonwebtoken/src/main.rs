use std::error::Error;

use r#const::EDDSA_JWT;
use decoder::decode_okp_ed25519;

mod r#const;
mod decoder;
mod entity;


fn main() -> Result<(), Box<dyn Error>> {
    let msg = decode_okp_ed25519(EDDSA_JWT)?;
    println!("{:?}", msg);
    Ok(())
}
