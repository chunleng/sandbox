use ory_client::models::{VerificationFlow,UiNodeAttributes};
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use crate::config::API_BASE_URL;
use serde_json::json;

#[derive(Clone)]
pub enum GetVerifyFlowError {
    Gone410,
    Unknown,
}

pub async fn get_verify_flow(
    flow_id: String,
) -> Result<VerificationFlow, GetVerifyFlowError> {
    let client = reqwest::Client::builder().build().unwrap();
    let req = client
        .get(format!(
            "{}/self-service/verification/flows?id={}",
            *API_BASE_URL, flow_id
        ))
        .fetch_credentials_include()
        .send();
    let res = req.await.map_err(|_| GetVerifyFlowError::Unknown)?;

    if res.status() == StatusCode::GONE {
        return Err(GetVerifyFlowError::Gone410);
    } else if !res.status().is_success() {
        return Err(GetVerifyFlowError::Unknown);
    }

    let t = res.text().await;
    Ok(serde_json::from_str::<VerificationFlow>(&t.map_err(|_| {
        GetVerifyFlowError::Unknown
    })?)
    .map_err(|_| GetVerifyFlowError::Unknown)?)
}

#[derive(Clone)]
pub enum VerifyError {
    BadRequest400,
    Gone410,
    Unknown,
}
pub async fn verify(email: Option<String>, code: Option<String>, flow: VerificationFlow) -> Result<(), VerifyError> {
    let mut headers = HeaderMap::new();
    // This header is necessary so that we have a API response returned instead of a web page
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let csrf_token = flow
        .ui
        .nodes
        .iter()
        .filter_map(|x| {
            if let UiNodeAttributes::Input(y) = x.attributes.as_ref() {
                if y.name == "csrf_token" {
                    return Some(
                        y.value
                            .clone()
                            .unwrap()
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string(),
                    );
                }
            }
            None
        })
        .last()
        .unwrap();
    match client
        .post(format!(
            "{}/self-service/verification?flow={}",
            *API_BASE_URL, flow.id
        ))
        .body(json!({
            "csrf_token": csrf_token,
            "code": code,
            "method": "code",
            "email": email
        }).to_string())
        .fetch_credentials_include()
        .send()
        .await
    {
        Ok(res) => match res.status() {
            StatusCode::OK => Ok(()),
            StatusCode::BAD_REQUEST => Err(VerifyError::BadRequest400),
            StatusCode::GONE => Err(VerifyError::Gone410),
            _ => Err(VerifyError::Unknown),
        },
        _ => Err(VerifyError::Unknown),
    }
}
