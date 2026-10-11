use sockudo_ws::{Http1, Message, SplitReader, Stream};
use tokio::sync::mpsc::Sender;

use crate::messages::MessageResponse;

#[tracing::instrument(name = "handle_incoming_messages", skip_all)]
pub async fn handle_incoming(mut rx: SplitReader<Stream<Http1>>, tx: Sender<MessageResponse>) {
    while let Some(res) = rx.next().await {
        let msg = match res {
            Ok(Message::Text(contents)) => {
                let response = serde_json::from_slice::<MessageResponse>(&contents);
                if let Ok(message) = response {
                    message
                } else {
                    let err = response.unwrap_err();
                    tracing::warn!("Received malformed frame: {err:#?}");

                    continue;
                }
            }

            Err(e) => {
                tracing::error!("Encountered websocket error: {e}");
                break;
            }
            _ => continue, // Ping/Pong handled by sockudo-ws
        };

        if let Err(err) = tx.send(msg).await {
            tracing::warn!("Failed to send response to receiver: {err}")
        }
    }

    tracing::debug!("Connection closed")
}
