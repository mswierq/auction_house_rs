use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional token to use for authentication, if not provided, the cli will read the token from the environment variable AUCTION_HOUSE_TOKEN
    #[arg(short, long)]
    pub token: Option<String>,

    /// Command to execute
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// User session commands
    User {
        #[command(subcommand)]
        command: UserCommands,
    },
    /// Funds commands
    Funds {
        #[command(subcommand)]
        command: FundsCommands,
    },
    /// Items commands
    Items {
        #[command(subcommand)]
        command: ItemsCommands,
    },
    /// Auctions commands
    Auctions {
        #[command(subcommand)]
        command: AuctionsCommands,
    },
}

#[derive(Subcommand)]
pub enum UserCommands {
    /// Register a new user and return a token, fails if a username is already taken or if already logged in
    Register {
        /// new user's username
        #[arg(short, long)]
        username: String,

        /// new user's password
        #[arg(short, long)]
        password: String,
    },
    /// Login with an existing user and return a token, takes no effect if already logged in
    Login {
        /// existing user's username
        #[arg(short, long)]
        username: String,

        /// existing user's password
        #[arg(short, long)]
        password: String,
    },
    /// Logout and invalidate the current token, takes no effect if not logged in
    Logout,
    /// Delete the current user and invalidate the current token, fails if not logged in
    Delete,
    /// Change the current user's password and return a new token, fails if not logged in
    ChangePassword {
        /// current user's password
        #[arg(short, long)]
        old_password: String,

        /// new user's password
        #[arg(short, long)]
        new_password: String,
    },
    /// Refresh the current token, fails if not logged in or if the token is invalid
    RefreshToken,
}

#[derive(Subcommand)]
pub enum FundsCommands {
    /// Get the current user's balance, fails if not logged in
    Balance,
    /// Deposit funds into the current user's balance, fails if not logged in
    Deposit {
        /// amount to deposit
        #[arg(short, long)]
        amount: u64,
    },
    /// Withdraw funds from the current user's balance, fails if not logged in or if the balance is insufficient
    Withdraw {
        /// amount to withdraw
        #[arg(short, long)]
        amount: u64,
    },
}

#[derive(Subcommand)]
pub enum ItemsCommands {
    /// List the current user's items, fails if not logged in
    List,
    /// Create a new item and add it to the current user's items, fails if not logged in
    Deposit {
        /// item's name
        #[arg(short, long)]
        name: String,
    },
    /// Remove an item from the current user's items, fails if not logged in or if the item does not exist
    Withdraw {
        /// item's name
        #[arg(short, long)]
        name: String,
    },
}

#[derive(Subcommand)]
pub enum AuctionsCommands {
    /// List all auctions, token is not required
    List {
        /// watch the auctions list for changes
        #[arg(short, long, action = clap::ArgAction::SetTrue)]
        watch: bool,
    },
    /// Create a new auction, fails if not logged in
    Create {
        /// item's name
        #[arg(short, long)]
        item: String,

        /// starting price
        #[arg(short, long)]
        starting_price: u64,

        /// duration in seconds
        #[arg(short, long)]
        duration: u64,
    },
    /// Bid on an auction, fails if not logged in or if the auction does not exist
    Bid {
        /// auction's id
        #[arg(short, long)]
        auction_id: u64,

        /// bid amount, fails if the amount is lower than the current bid or the starting price
        #[arg(short, long)]
        amount: u64,
    },
    /// Close an auction, fails if not logged in, the auction does not exist or user is not the auction's owner
    Close {
        /// auction's id
        #[arg(short, long)]
        auction_id: u64,
    },
    /// Watch all user's auctions, fails if not logged in, refreshes user's token if it's expired
    Watch,
}
