use anyhow::{Error, Result};
use clap::{Parser, Subcommand};
use ory_client::{
    apis::{
        configuration::Configuration,
        frontend_api::{
            create_native_login_flow, create_native_registration_flow, update_login_flow,
            update_registration_flow, UpdateLoginFlowError, UpdateRegistrationFlowError,
        },
        Error as OryError,
    },
    models::{
        UiNodeAttributes, UpdateLoginFlowBody, UpdateLoginFlowWithPasswordMethod,
        UpdateRegistrationFlowBody, UpdateRegistrationFlowWithPasswordMethod,
    },
};
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
    }

    Ok(())
}

async fn register(email: String, password: String) -> Result<()> {
    let config = Configuration {
        base_path: "http://localhost:4433".to_string(),
        ..Default::default()
    };
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
        let mut context = None;
        if let OryError::ResponseError(e) = &err {
            if let Some(UpdateRegistrationFlowError::Status400(e)) = &e.entity {
                if let Some(e) = &e.ui.messages {
                    context = e
                        .iter()
                        .map(|x| format!("{} (code: {})", x.text, x.id))
                        .reduce(|a, b| format!("{}\n{}", a, b));
                } else {
                    context =
                        e.ui.nodes
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
                            .reduce(|a, b| format!("{}\n{}", a, b));
                }
            }
        }
        Error::from(err).context(context.unwrap_or("Unknown Error".to_string()))
    })?;

    println!("Registered user: {}", email);
    Ok(())
}

async fn login(email: String, password: String) -> Result<()> {
    let config = Configuration {
        base_path: "http://localhost:4433".to_string(),
        ..Default::default()
    };
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
        let mut context = None;
        if let OryError::ResponseError(e) = &err {
            if let Some(UpdateLoginFlowError::Status400(e)) = &e.entity {
                if let Some(e) = &e.ui.messages {
                    context = e
                        .iter()
                        .map(|x| format!("{} (code: {})", x.text, x.id))
                        .reduce(|a, b| format!("{}\n{}", a, b));
                } else {
                    context =
                        e.ui.nodes
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
                            .reduce(|a, b| format!("{}\n{}", a, b));
                }
            }
        }
        Error::from(err).context(context.unwrap_or("Unknown Error".to_string()))
    })?;

    println!("Logged in user: {}", email);
    Ok(())
}
