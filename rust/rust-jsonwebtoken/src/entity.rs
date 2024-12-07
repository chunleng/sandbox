use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub email: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub data: Data,
    pub exp: u32,
    pub iat: u32,
    pub iss: String,
    pub jti: String,
    pub nbf: u32,
    pub sid: String,
    pub sub: String,
}
