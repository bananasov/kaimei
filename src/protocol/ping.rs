use std::time::{Duration, Instant};

use iroh::{
    Endpoint, EndpointAddr,
    endpoint::Connection,
    protocol::{AcceptError, ProtocolHandler},
};

pub const ALPN: &[u8] = b"kaimei/ping/0";

#[derive(Debug, Clone)]
pub struct Ping {}

impl Ping {
    #[inline(always)]
    pub fn new() -> Self {
        Self {}
    }

    pub async fn ping(&self, endpoint: &Endpoint, addr: EndpointAddr) -> anyhow::Result<Duration> {
        let conn = endpoint.connect(addr, ALPN).await?;

        let (mut send, mut recv) = conn.open_bi().await?;

        let start = Instant::now();
        send.write_all(b"PING").await?;

        send.finish()?;

        let response = recv.read_to_end(4).await?;
        assert_eq!(&response, b"PONG");

        let ping = start.elapsed();

        conn.close(0u32.into(), b"bye!");
        Ok(ping)
    }
}

impl ProtocolHandler for Ping {
    /// The `accept` method is called for each incoming connection for our ALPN.
    ///
    /// The returned future runs on a newly spawned tokio task, so it can run as long as
    /// the connection lasts.
    #[tracing::instrument(skip_all)]
    async fn accept(&self, connection: Connection) -> anyhow::Result<(), AcceptError> {
        let node_id = connection.remote_id();
        tracing::info!("accepted connection from {node_id}");

        let (mut send, mut recv) = connection.accept_bi().await?;
        let req = recv.read_to_end(4).await.map_err(AcceptError::from_err)?;
        assert_eq!(&req, b"PING");

        send.write_all(b"PONG")
            .await
            .map_err(AcceptError::from_err)?;

        send.finish()?;

        connection.closed().await;
        Ok(())
    }
}
