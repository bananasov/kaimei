//! Messages used for client/server connection lifecycle, including handshake and disconnection.

use serde::{Deserialize, Serialize};

/// Sent by the client to register itself with the server, and request info from the server.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequestServerInfoMessage {
    pub id: i32,

    /// Name of the client, for the server to use for UI if needed. Cannot be null.
    pub client_name: String,

    /// Message spec major version of the client software.
    pub protocol_version_major: u32,

    /// Message spec minor version of the client software.
    pub protocol_version_minor: u32,
}

/// Sent by server to client, contains information about the server name (optional) and ping time expectations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServerInfoMessage {
    pub id: i32,

    /// Name of the client, for the server to use for UI if needed. Cannot be null.
    pub server_name: String,

    /// Maximum interval for pings from the client, in milliseconds.
    ///
    /// If a client takes longer than this time between sending Ping messages,
    /// the server is expected to disconnect.
    ///
    /// A value of 0 means the server does not require Ping messages for this connection.
    pub max_ping_time: u32,

    /// Protocol major version of the server for this connection.
    pub protocol_version_major: u32,

    /// Protocol minor version of the server for this connection.
    pub protocol_version_minor: u32,
}

/// Sent by the client to request a graceful disconnection from the server.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DisconnectMessage {
    pub id: i32,
}
