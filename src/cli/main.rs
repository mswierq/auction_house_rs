use clap::Parser;
use client_session_proto::client_session_client::ClientSessionClient;
use client_session_proto::{ChangePasswordRequest, LoginRequest, RegisterRequest};
use std::env;

pub mod client_session_proto {
    tonic::include_proto!("auction_house_rs.session.client");
}
mod commands;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = commands::Cli::parse();

    let token = if let Some(token) = cli.token {
        Some(token)
    } else if let Ok(token) = env::var("AUCTION_HOUSE_TOKEN") {
        Some(token)
    } else {
        None
    };

    // TODO: refactor this
    let result = match &cli.command {
        commands::Commands::User { command } => {
            let mut client = ClientSessionClient::connect("http://[::1]:50051").await?;
            match &command {
                commands::UserCommands::Register { username, password } => {
                    let request = tonic::Request::new(RegisterRequest {
                        username: username.clone(),
                        password: password.clone(),
                    });
                    let response = client.register(request).await?;
                    format!("Your token is: {}", response.into_inner().token)
                }
                commands::UserCommands::Login { username, password } => {
                    let request = tonic::Request::new(LoginRequest {
                        username: username.clone(),
                        password: password.clone(),
                    });
                    let response = client.login(request).await?;
                    format!("Your token is: {}", response.into_inner().token)
                }
                commands::UserCommands::Logout => {
                    let request = tonic::Request::new(());
                    let _ = client.logout(request).await?;
                    format!("You have been logged out")
                }
                commands::UserCommands::Delete => {
                    let request = tonic::Request::new(());
                    let _ = client.logout(request).await?;
                    format!("Your account has been deleted")
                }
                commands::UserCommands::ChangePassword {
                    old_password,
                    new_password,
                } => {
                    let request = tonic::Request::new(ChangePasswordRequest {
                        old_password: old_password.clone(),
                        new_password: new_password.clone(),
                    });
                    let response = client.change_password(request).await?;
                    format!("Your token is: {}", response.into_inner().token)
                }
                commands::UserCommands::RefreshToken => {
                    let request = tonic::Request::new(());
                    let response = client.refresh_token(request).await?;
                    format!("Your token is: {}", response.into_inner().token)
                }
            }
        }
        commands::Commands::Funds { command } => {
            unimplemented!()
        }
        commands::Commands::Items { command } => {
            unimplemented!()
        }
        commands::Commands::Auctions { command } => {
            unimplemented!()
        }
    };

    println!("{}", result);

    Ok(())
}
