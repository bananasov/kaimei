//! Messages for controlling devices.

pub mod value;

use serde::{Deserialize, Serialize};
use value::{OutputHwPositionWithDuration, OutputValue};

/// Sends a command to an output of some type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct OutputCmdMessage {
    pub id: u32,
    pub device_index: u32,
    pub feature_index: u32,

    pub command: OutputCommand,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum OutputCommand {
    Vibrate(OutputValue),
    Rotate(OutputValue),
    Oscillate(OutputValue),
    Constrict(OutputValue),
    Spray(OutputValue),
    Temperature(OutputValue),
    Led(OutputValue),
    Position(OutputValue),
    HwPositionWithDuration(OutputHwPositionWithDuration),
}
