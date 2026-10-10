pub mod connection;
pub mod device;
pub mod status;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum Message {
    RequestServerInfo(connection::RequestServerInfoMessage),
    Ping(status::PingMessage),

    StartScanning(device::StartScanningMessage),
    StopScanning(device::StopScanningMessage),

    Stop(device::StopMessage),
    OutputCmd(device::output::OutputCmdMessage),
    Input(device::input::InputCmdMessage),

    Disconnect(connection::DisconnectMessage),
}
