use serde_json::{Value::Object, json};
use sockudo_ws::{Config, Http1, Message, client::WebSocketClient};
use tokio::net::TcpStream;

use crate::messages::Message as ButtplugMessage;
use crate::messages::connection::RequestServerInfoMessage;

pub struct ButtplugClient {}

impl<'a> ButtplugClient {
    pub async fn new(client_name: &'a str, host: &'a str, path: &'a str) -> anyhow::Result<()> {
        let stream = TcpStream::connect(host).await?; // TODO: DO NOT UNWRAP
        let client = WebSocketClient::<Http1>::new(Config::default());
        let (ws, _handshake) = client.connect(stream, host, path, None).await?;
        let (mut reader, mut writer) = ws.split();

        let request = RequestServerInfoMessage {
            id: 1,
            client_name: client_name.into(),
            protocol_version_major: 4,
            protocol_version_minor: 0,
        };
        let msg = ButtplugMessage::RequestServerInfo(request);
        let meow = vec![msg];
        let json = serde_json::to_string(&meow)?;

        writer.send_text(json).await?;

        let mwu = reader.next().await;
        if let Some(message) = mwu
            && let Ok(mrro) = message
            && let Message::Text(contents) = mrro
        {
            // SAFETY: WebSocket text messages SHOULD be encoded as UTF-8.
            let string = unsafe { str::from_utf8_unchecked(&contents) };
            tracing::info!("{string}");
        }

        writer.close(0, "exiting").await?;

        Ok(())
    }
}
