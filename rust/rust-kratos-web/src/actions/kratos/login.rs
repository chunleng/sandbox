use ory_client::models::{LoginFlow, UiNodeAttributes};
use reqwest::{header::HeaderMap, StatusCode};
use serde_json::json;

use crate::config::API_BASE_URL;


#[derive(Debug, Clone)]
pub enum GetLoginFlowError {
    Gone410,
    Unknown,
}


pub async fn get_login_flow(
    flow_id: String,
) -> Result<LoginFlow, GetLoginFlowError> {
    let client = reqwest::Client::builder().build().unwrap();
    let req = client
        .get(format!(
            "{}/self-service/login/flows?id={}",
            *API_BASE_URL, flow_id
        ))
        .fetch_credentials_include()
        .send();
    let res = req.await.map_err(|_| GetLoginFlowError::Unknown)?;

    if res.status() == StatusCode::GONE {
        return Err(GetLoginFlowError::Gone410);
    } else if !res.status().is_success() {
        return Err(GetLoginFlowError::Unknown);
    }

    let t = res.text().await;
    Ok(serde_json::from_str::<LoginFlow>(&t.map_err(|_| {
        GetLoginFlowError::Unknown
    })?)
    .map_err(|_| GetLoginFlowError::Unknown)?)
}

#[derive(Debug, Clone)]
pub enum LoginError {
    Gone410,
    BadRequest400,
    Unknown,
}
pub async fn login(email: String, password: String, flow: LoginFlow) -> Result<(), LoginError> {
    let mut headers = HeaderMap::new();
    // This header is necessary so that we have a API response returned instead of a web page
    // headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("Content-Type", "application/json".parse().unwrap());
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();
    let csrf_token =
        flow.ui.nodes
            .iter()
            .flat_map(|x| {
                if let UiNodeAttributes::Input(y) = x.attributes.as_ref() {
                    if y.name == "csrf_token" {
                        return vec![y
                            .value
                            .clone()
                            .unwrap()
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .to_string()];
                    }
                }
                vec![]
            })
            .last()
            .unwrap();
    match client
        .post(format!(
            "{}/self-service/login?flow={}",
            *API_BASE_URL, flow.id
        ))
        .body(json!({
            "csrf_token": csrf_token,
            "method": "password",
            "identifier": email,
            "password": password,
        }).to_string())
        .fetch_credentials_include()
        .send()
        .await
    {
        Ok(res) => match res.status() {
            StatusCode::OK => Ok(()),
            StatusCode::BAD_REQUEST => Err(LoginError::BadRequest400),
            StatusCode::GONE => Err(LoginError::Gone410),
            _ => Err(LoginError::Unknown),
        },
        _ => Err(LoginError::Unknown),
    }
}
