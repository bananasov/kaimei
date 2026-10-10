use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(about, version)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Connect to a sub to control them
    Client {
        /// The server to connect to
        ticket: String,
    },

    /// Host a session to let others control you
    Server,
}
