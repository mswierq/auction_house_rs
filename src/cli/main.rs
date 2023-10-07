use clap::Parser;
use std::env;
mod commands;

fn main() {
    let cli = commands::Cli::parse();

    let token = if let Some(token) = cli.token {
        Some(token)
    } else if let Ok(token) = env::var("AUCTION_HOUSE_TOKEN") {
        Some(token)
    } else {
        None
    };

    match &cli.command {
        commands::Commands::User { command } => match &command {
            commands::UserCommands::Register { username, password } => {}
            commands::UserCommands::Login { username, password } => {}
            commands::UserCommands::Logout => {}
            commands::UserCommands::Delete => {}
            commands::UserCommands::ChangePassword {
                old_password,
                new_password,
            } => {}
            commands::UserCommands::RefreshToken => {}
        },
        commands::Commands::Funds { command } => match &command {
            commands::FundsCommands::Deposit { amount } => {}
            commands::FundsCommands::Withdraw { amount } => {}
            commands::FundsCommands::Balance => {}
        },
        commands::Commands::Items { command } => match &command {
            commands::ItemsCommands::List => {}
            commands::ItemsCommands::Deposit { name } => {}
            commands::ItemsCommands::Withdraw { name } => {}
        },
        commands::Commands::Auctions { command } => match &command {
            commands::AuctionsCommands::List { watch } => {}
            commands::AuctionsCommands::Create {
                item,
                starting_price,
                duration,
            } => {}
            commands::AuctionsCommands::Bid { auction_id, amount } => {}
            commands::AuctionsCommands::Close { auction_id } => {}
            commands::AuctionsCommands::Watch => {}
        },
    }
}
