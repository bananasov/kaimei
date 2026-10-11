//! Messages relaying different statuses, including communication statuses, connection (ping), log messages, etc...

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Signifies that the previous message sent by the client was received and processed successfully by the server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OkMessage {
    /// The Id of the client message that this reply is in response to.
    pub id: u32,
}

/// Signifies that the previous message sent by the client caused some sort of parsing or processing error on the server.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorMessage {
    /// The Id of the client message that this reply is in response to, assuming the Id could be parsed.
    ///
    /// Id will be 0 if message could not be parsed (due to issues like invalid JSON).
    pub id: u32,

    /// Message describing the error that happened on the server.
    pub error_message: String,

    /// Enum describing the error
    pub error_code: ErrorCode,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ErrorCode {
    /// An unknown error occurred.
    ErrorUnknown = 0,

    /// Handshake did not succeed.
    ErrorInit = 1,

    /// A ping was not sent in the expected time.
    ErrorPing = 2,

    /// A message parsing or permission error occurred.
    ErrorMsg = 3,

    /// A command sent to a device returned an error.
    ErrorDevice = 4,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PingMessage {
    pub id: u32,
}
