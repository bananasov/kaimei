use iroh::{Endpoint, endpoint::presets, protocol::Router};
use iroh_tickets::endpoint::EndpointTicket;

use crate::protocol::ping::{self, Ping};

#[tracing::instrument(skip_all)]
pub async fn listen() -> anyhow::Result<()> {
    let endpoint = Endpoint::bind(presets::N0).await?;
    endpoint.online().await;

    let ticket = EndpointTicket::new(endpoint.addr());
    tracing::info!("Ticket: {ticket}");

    let _router = Router::builder(endpoint.clone())
        .accept(ping::ALPN, Ping::new())
        .spawn();

    tokio::signal::ctrl_c().await?;

    endpoint.close().await;
    Ok(())
}
