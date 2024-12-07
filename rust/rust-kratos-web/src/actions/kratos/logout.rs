use crate::config::API_BASE_URL;
use leptos::window;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum LogoutError {
    Unknown,
}

#[derive(Deserialize)]
struct LogoutInfo {
    logout_url: String
}

pub async fn logout() -> Result<(), LogoutError> {
    let client = reqwest::Client::builder().build().unwrap();
    let req = client
        .get(format!(
            "{}/self-service/logout/browser",
            *API_BASE_URL
        ))
        .fetch_credentials_include()
        .send();
    let res = req.await.map_err(|_| LogoutError::Unknown)?;

    let logout_info =  res.json::<LogoutInfo>().await.map_err(|_| LogoutError::Unknown)?;

    if window().location().set_href(&logout_info.logout_url).is_err() {
        return Err(LogoutError::Unknown);
    }

    Ok(())
}
