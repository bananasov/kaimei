//! Messages that convey information about devices currently connected to the system.

use std::collections::{BTreeMap, HashMap};

use serde::{Deserialize, Serialize};

/// Server reply to a client request for a device list,
/// or sent as an event when a device is connected or disconnected.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceListMessage {
    pub id: u32,

    /// Map of device index to device info.
    pub devices: HashMap<u32, DeviceInfo>,
}

// TODO: Consider remove the `device_` prefix, seems redundant.
//       Would require me adding `#[serde(rename = "...")]` everywhere though.
/// Substructure of device messages, used for attribute information (name, messages supported, etc...)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceInfo {
    /// Descriptive name of the device
    pub device_name: String,

    /// Index used to identify the device when sending device messages.
    pub device_index: u32,

    /// Minimum gap between output command dispatches to this device, in milliseconds.
    pub device_message_timing_gap: u32,

    /// User provided display name for a device.
    ///
    /// Useful for cases where a users may have multiple of the same device connected.
    pub device_display_name: String,

    pub device_features: BTreeMap<u32, DeviceFeature>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceFeature {
    pub feature_description: String,
    pub feature_index: i32,

    // Genuinely an insane field, can't be bothered to completely document it.
    //
    // PRs welcome!
    // NOTE: Might be able to remove the Option<T>, Value has a null variant.
    /// Represents outputs that are part of this feature.
    pub output: Option<serde_json::Value>,

    // NOTE: Same as above, can't be bothered.
    /// Represents inputs that may be part of this feature.
    pub input: Option<serde_json::Value>,
}
