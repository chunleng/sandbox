use ory_client::models::RegistrationFlow;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;

use crate::config::API_BASE_URL;
use serde_json::json;

#[derive(Debug, Clone)]
pub enum GetRegisterFlowError {
    Gone410,
    Unknown,
}

pub async fn get_registration_flow(
    flow_id: String,
) -> Result<RegistrationFlow, GetRegisterFlowError> {
    let client = reqwest::Client::builder().build().unwrap();
    let req = client
        .get(format!(
            "{}/self-service/registration/flows?id={}",
            *API_BASE_URL, flow_id
        ))
        .fetch_credentials_include()
        .send();
    let res = req.await.map_err(|_| GetRegisterFlowError::Unknown)?;

    if res.status() == StatusCode::GONE {
        return Err(GetRegisterFlowError::Gone410);
    } else if !res.status().is_success() {
        return Err(GetRegisterFlowError::Unknown);
    }

    let t = res.text().await;
    Ok(serde_json::from_str::<RegistrationFlow>(&t.map_err(|_| {
        GetRegisterFlowError::Unknown
    })?)
    .map_err(|_| GetRegisterFlowError::Unknown)?)
}

#[derive(Debug, Clone)]
pub enum RegisterError {
    ValidationError { messages: Vec<String> },
    Unknown,
}
pub async fn register(email: String, password: String, flow_id: String, csrf_token: String) -> Result<(), RegisterError> {
    let mut headers = HeaderMap::new();
    // This header is necessary so that we have a API response returned instead of a web page
    // headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    match client
        .post(format!(
            "{}/self-service/registration?flow={}",
            *API_BASE_URL, flow_id
        ))
        .body(json!({
            "csrf_token": csrf_token,
            "method": "password",
            "password": password,
            "traits": {"email": email}
        }).to_string())
        .fetch_credentials_include()
        .send()
        .await
    {
        Ok(res) => match res.status() {
            StatusCode::OK => Ok(()),
            StatusCode::BAD_REQUEST => {
                let r = get_registration_flow(flow_id.to_string()).await;
                match r {
                    Ok(flow) => {
                        let mut messages: Vec<String> = flow
                            .ui
                            .nodes
                            .iter()
                            .flat_map(|x| x.messages.iter().map(|x| x.text.clone()))
                            .collect();
                        if messages.len() == 0 {
                            messages = flow.ui.messages.iter().flat_map(|x| x.iter().map(|x| x.text.clone())).collect();
                        }
                        Err(RegisterError::ValidationError { messages })
                    }
                    _ => Err(RegisterError::Unknown),
                }
            }
            _ => Err(RegisterError::Unknown),
        },
        _ => Err(RegisterError::Unknown),
    }
}
