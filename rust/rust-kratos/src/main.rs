use anyhow::Result;
use clap::{Parser, Subcommand};
use custom_error::custom_error;
use lazy_static::lazy_static;
use ory_client::{
    apis::{
        configuration::Configuration,
        frontend_api::{
            create_native_login_flow, create_native_registration_flow, update_login_flow,
            update_registration_flow, update_verification_flow, UpdateLoginFlowError,
            UpdateRegistrationFlowError, UpdateVerificationFlowError,
        },
        Error as OryError,
    },
    models::{
        ui_text::TypeEnum, UiNodeAttributes, UpdateLoginFlowBody,
        UpdateLoginFlowWithPasswordMethod, UpdateRegistrationFlowBody,
        UpdateRegistrationFlowWithPasswordMethod, UpdateVerificationFlowBody,
        UpdateVerificationFlowWithCodeMethod,
    },
};
use reqwest::{header::HeaderMap, Client};
use serde_json::json;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    REGISTER {
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        password: String,
    },
    LOGIN {
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        password: String,
    },
    VERIFY {
        #[arg(short, long)]
        flow_id: String,
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        code: String,
    },
}

lazy_static! {
    static ref config: Configuration = {
        let mut headers = HeaderMap::new();
        // This header is necessary so that we have a API response returned instead of a web page
        headers.insert("Accept", "application/json".parse().unwrap());
        let client = Client::builder().default_headers(headers).build().unwrap();
        Configuration {
            base_path: "http://localhost:4433".to_string(),
            client,
            ..Default::default()
        }
    };
}

custom_error! {ApplicationError
    KnownError{message: String} = "{message}",
    UnknownError = "An unknown error has occurred"
}

#[tokio::main]
async fn main() {
    if let Err(e) = process_command().await {
        println!("{:?}", e);
    }
}

async fn process_command() -> Result<()> {
    let args = Cli::try_parse()?;
    match args.command {
        Commands::REGISTER { email, password } => {
            register(email, password).await?;
        }
        Commands::LOGIN { email, password } => {
            login(email, password).await?;
        }
        Commands::VERIFY {
            flow_id,
            email,
            code,
        } => {
            verify(flow_id, email, code).await?;
        }
    }

    Ok(())
}

async fn register(email: String, password: String) -> Result<()> {
    let registration_flow = create_native_registration_flow(&config, None, None).await?;

    update_registration_flow(
        &config,
        &registration_flow.id,
        UpdateRegistrationFlowBody::Password(Box::new(UpdateRegistrationFlowWithPasswordMethod {
            password,
            traits: json!({"email": email}),
            ..Default::default()
        })),
        None,
    )
    .await
    .map_err(|err| {
        if let OryError::ResponseError(e) = &err {
            if let Some(UpdateRegistrationFlowError::Status400(e)) = &e.entity {
                if let Some(e) = &e.ui.messages {
                    return match e
                        .iter()
                        .map(|x| format!("{} (code: {})", x.text, x.id))
                        .reduce(|a, b| format!("{}\n{}", a, b))
                    {
                        Some(message) => ApplicationError::KnownError { message },
                        _ => ApplicationError::UnknownError,
                    };
                } else {
                    return match e
                        .ui
                        .nodes
                        .iter()
                        .flat_map(|x| {
                            let mut name = None;
                            if let UiNodeAttributes::Input(ii) = &*x.attributes {
                                name = Some(&ii.name);
                            }
                            x.messages.iter().map(move |x| {
                                format!(
                                    "In {}: {} (code: {})",
                                    name.unwrap_or(&"".to_string()),
                                    x.text,
                                    x.id
                                )
                            })
                        })
                        .reduce(|a, b| format!("{}\n{}", a, b))
                    {
                        Some(message) => ApplicationError::KnownError { message },
                        _ => ApplicationError::UnknownError,
                    };
                }
            }
        }
        ApplicationError::UnknownError
    })?;

    println!("Registered user: {}", email);
    Ok(())
}

async fn login(email: String, password: String) -> Result<()> {
    let login_flow = create_native_login_flow(&config, None, None, None, None, None, None).await?;

    update_login_flow(
        &config,
        &login_flow.id,
        UpdateLoginFlowBody::Password(Box::new(UpdateLoginFlowWithPasswordMethod {
            identifier: email.clone(),
            password,
            ..Default::default()
        })),
        None,
        None,
    )
    .await
    .map_err(|err| {
        if let OryError::ResponseError(e) = &err {
            if let Some(UpdateLoginFlowError::Status400(e)) = &e.entity {
                if let Some(e) = &e.ui.messages {
                    return match e
                        .iter()
                        .map(|x| format!("{} (code: {})", x.text, x.id))
                        .reduce(|a, b| format!("{}\n{}", a, b))
                    {
                        Some(message) => ApplicationError::KnownError { message },
                        _ => ApplicationError::UnknownError,
                    };
                } else {
                    return match e
                        .ui
                        .nodes
                        .iter()
                        .flat_map(|x| {
                            let mut name = None;
                            if let UiNodeAttributes::Input(ii) = &*x.attributes {
                                name = Some(&ii.name);
                            }
                            x.messages.iter().map(move |x| {
                                format!(
                                    "In {}: {} (code: {})",
                                    name.unwrap_or(&"".to_string()),
                                    x.text,
                                    x.id
                                )
                            })
                        })
                        .reduce(|a, b| format!("{}\n{}", a, b))
                    {
                        Some(message) => ApplicationError::KnownError { message },
                        _ => ApplicationError::UnknownError,
                    };
                }
            }
        }
        ApplicationError::UnknownError
    })?;

    println!("Logged in user: {}", email);
    Ok(())
}

async fn verify(flow_id: String, email: String, code: String) -> Result<()> {
    let res = update_verification_flow(
        &config,
        &flow_id,
        UpdateVerificationFlowBody::Code(Box::new(UpdateVerificationFlowWithCodeMethod {
            email: Some(email.clone()),
            code: Some(code),
            ..Default::default()
        })),
        None,
        None,
    )
    .await
    .map_err(|err| {
        if let OryError::ResponseError(e) = &err {
            if let Some(UpdateVerificationFlowError::Status410(e)) = &e.entity {
                if let Some(e) = &e.error.message {
                    return ApplicationError::KnownError {
                        message: e.to_string(),
                    };
                }
            }
        }
        ApplicationError::UnknownError
    })?;

    if let Some(messages) = &res.ui.messages {
        if let Some(message) = messages
            .iter()
            .filter(|x| x.r#type == TypeEnum::Error)
            .map(|x| format!("{} (code: {})", x.text, x.id))
            .reduce(|a, b| format!("{}\n{}", a, b))
        {
            return Err(ApplicationError::KnownError { message }.into());
        }
    }

    println!("Verified User: {}", email);
    Ok(())
}
