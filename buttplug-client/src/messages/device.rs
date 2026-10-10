//! Messages relating to finding and getting information about devices connected to the system.

pub mod information;
pub mod input;
pub mod output;

use serde::{Deserialize, Serialize};

/// Client request to have the server start scanning for devices on all busses that it knows about.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StartScanningMessage {
    pub id: i32,
}

/// Client request to have the server stop scanning for devices.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopScanningMessage {
    pub id: i32,
}

/// Sent by the server once it has stopped scanning on all busses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ScanningFinishedMessage {
    pub id: i32,
}

/// Client request to have the server send over its known device list, without starting a full scan.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RequestDeviceListMessage {
    pub id: i32,
}

/// Client request to have the server stop all devices, a device,
/// or a feature on a device from whatever actions it may be taking.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopMessage {
    pub id: i32,

    /// Index of device to stop. If not included, all devices are stopped.
    pub device_index: Option<u32>,

    /// Index of the feature on the device to stop, assuming as device index is specified.
    ///
    /// f no device index is specified, feature index is ignored.
    pub feature_index: Option<u32>,

    /// If true, unsubscribe subscribed inputs based on index selection.
    #[serde(default = "default_true")]
    pub inputs: Option<bool>,

    /// If true, stop outputs based on index selection.
    #[serde(default = "default_true")]
    pub outputs: Option<bool>,
}

#[inline]
fn default_true() -> Option<bool> {
    Some(true)
}
