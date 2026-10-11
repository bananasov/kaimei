mod handle;

use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

use anyhow::{Context, bail};
use sockudo_ws::error::CloseReason;
use sockudo_ws::{Config, Http1, Message, client::WebSocketClient};
use sockudo_ws::{SplitWriter, Stream};
use tokio::sync::Mutex;
use tokio::sync::mpsc::{self, Receiver};

use crate::messages::{self, MessageRequest, MessageResponse};

#[derive(Clone)]
pub struct ButtplugClient {
    last_message_id: Arc<AtomicU32>,
    tx: Arc<Mutex<SplitWriter<Stream<Http1>>>>,
}

impl<'a> ButtplugClient {
    pub async fn new(
        client_name: &'a str,
        url: &'a str,
    ) -> anyhow::Result<(Self, Receiver<MessageResponse>)> {
        let client = WebSocketClient::<Http1>::new(Config::default());
        let (ws, _handshake) = client.connect_to_url(url, None).await?;

        let (send, recv) = mpsc::channel::<messages::MessageResponse>(64);
        let (rx, tx) = ws.split();

        let res = Self {
            last_message_id: Arc::new(AtomicU32::from(1)),
            tx: Arc::new(Mutex::new(tx)),
        };

        tokio::spawn(handle::handle_incoming(rx, send));

        let id = res.last_message_id.load(Ordering::Relaxed);
        let mrrp = MessageRequest::request_server_info(id, client_name);

        res.send_request(mrrp).await?;

        Ok((res, recv))
    }

    fn next_id(&self) -> u32 {
        self.last_message_id.fetch_add(1, Ordering::Relaxed)
    }

    // TODO: Response handling
    async fn send_request(&self, req: MessageRequest) -> anyhow::Result<()> {
        let mut tx = self.tx.lock().await;

        let msg = [req];
        let value = serde_json::to_string(&msg).context("Failed to serialize message request")?;

        if let Err(err) = tx.send_text(value).await {
            tracing::error!("Couldn't send request");

            bail!(err);
        }

        drop(tx);
        Ok(())
    }

    pub async fn close(self) -> anyhow::Result<()> {
        use messages::connection::DisconnectMessage;

        tracing::debug!("Closing websocket connection");

        let mut tx = self.tx.lock().await;

        let id = self.next_id();
        let req = MessageRequest::Disconnect(DisconnectMessage { id });
        self.send_request(req).await?;

        tx.close(CloseReason::NORMAL, "closing").await?;

        drop(tx);
        Ok(())
    }
}
