//! Messages for receiving sensor input from devices.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InputCmdMessage {
    pub id: u32,
    pub device_index: u32,
    pub device_feature: u32,

    /// The type of input we expect to receive from this feature.
    #[serde(rename = "Type")]
    pub input_type: InputType,

    pub command: InputCommand,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputCommand {
    Read,
    Subscribe,
    Unsubscribe,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InputReadingMessage {
    pub id: u32,
    pub device_index: u32,
    pub feature_index: u32,

    /// Data from the input, including `InputType` and the corresponding format.
    pub reading: InputData,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum InputData {
    Battery(InputValue<u8>),
    Rssi(InputValue<i8>),
    Button(InputValue<u8>),
    Pressure(InputValue<u32>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum InputType {
    Battery,
    Rssi,
    Button,
    Pressure,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InputValue<T> {
    value: T,
}

impl<T> InputValue<T> {
    pub const fn new(value: T) -> Self {
        Self { value }
    }
}
