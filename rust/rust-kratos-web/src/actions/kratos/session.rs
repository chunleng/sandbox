use reqwest::StatusCode;

use crate::config::API_BASE_URL;

#[derive(Debug, Clone)]
pub enum WhoAmIError {
    Unauthorized401,
    Unknown,
}

pub async fn whoami() -> Result<String, WhoAmIError> {
    let client = reqwest::Client::builder().build().unwrap();
    let req = client
        .get(format!(
            "{}/sessions/whoami?tokenize_as=backend",
            *API_BASE_URL
        ))
        .fetch_credentials_include()
        .send();
    let res = req.await.map_err(|_| WhoAmIError::Unknown)?;

    if res.status() == StatusCode::UNAUTHORIZED {
        return Err(WhoAmIError::Unauthorized401);
    }

    Ok(res.text().await.map_err(|_| WhoAmIError::Unknown)?)
}
