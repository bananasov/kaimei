mod args;
mod client;
mod protocol;
mod server;

use clap::Parser;
use iroh::{Endpoint, endpoint::presets};
use iroh_tickets::{Ticket, endpoint::EndpointTicket};

use crate::protocol::ping::Ping;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let args = args::Args::parse();

    match args.command {
        args::Command::Client { ticket } => run_client(ticket).await?,
        args::Command::Server => server::listen::listen().await?,
    }

    Ok(())
}

async fn run_client(ticket: String) -> anyhow::Result<()> {
    // EndpointTicket::deserialize did not want to work for some reason?
    // Either way, this saves some instructions at least.
    let ticket = EndpointTicket::decode_string(&ticket)
        .map_err(|e| anyhow::anyhow!("failed to parse ticket: {}", e))?;

    let client = Endpoint::bind(presets::N0).await?;
    let pinger = Ping::new();

    let rtt = pinger.ping(&client, ticket.endpoint_addr().clone()).await?;
    tracing::info!("ping took: {:?} to complete", rtt);

    client.close().await;
    Ok(())
}
