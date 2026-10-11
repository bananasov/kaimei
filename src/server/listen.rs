use buttplug_client::{client::ButtplugClient, messages::MessageResponse};
use iroh::{Endpoint, endpoint::presets, protocol::Router};
use iroh_tickets::endpoint::EndpointTicket;
use tokio::sync::mpsc::Receiver;

use crate::protocol::ping::{self, Ping};

#[tracing::instrument(skip_all)]
pub async fn listen() -> anyhow::Result<()> {
    let (client, receiver) = ButtplugClient::new("kaimei", "ws://127.0.0.1:12345/").await?;
    tokio::spawn(handle_buttplug(receiver));
    tracing::info!("Connected to buttplug");

    let endpoint = Endpoint::bind(presets::N0).await?;
    endpoint.online().await;

    let ticket = EndpointTicket::new(endpoint.addr());
    tracing::info!("Ticket: {ticket}");

    let _router = Router::builder(endpoint.clone())
        .accept(ping::ALPN, Ping::new())
        .spawn();

    tokio::signal::ctrl_c().await?;

    endpoint.close().await;
    client.close().await?;
    Ok(())
}

#[tracing::instrument(skip_all)]
async fn handle_buttplug(mut rx: Receiver<MessageResponse>) {
    while let Some(msg) = rx.recv().await {
        match msg {
            MessageResponse::Ok(_) => tracing::debug!("Received OK message from Intiface"),
            MessageResponse::Error(message) => tracing::error!(
                "Received an error: {}: {}",
                message.error_code,
                message.error_message
            ),
            MessageResponse::ServerInfo(message) => tracing::info!("{:#?}", message),
            MessageResponse::DeviceList(message) => tracing::info!("{:#?}", message),
            _ => {}
        }
    }
}
