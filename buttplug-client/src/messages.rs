pub mod connection;
pub mod device;
pub mod status;

use serde::{Deserialize, Serialize};

const CURRENT_SUPPORTED_VERSION_MAJOR: u32 = 4;
const CURRENT_SUPPORTED_VERSION_MINOR: u32 = 0;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MessageRequest {
    RequestServerInfo(connection::RequestServerInfoMessage),
    Ping(status::PingMessage),

    StartScanning(device::StartScanningMessage),
    StopScanning(device::StopScanningMessage),

    Stop(device::StopMessage),
    OutputCmd(device::output::OutputCmdMessage),
    Input(device::input::InputCmdMessage),

    Disconnect(connection::DisconnectMessage),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MessageResponse {
    Ok(status::OkMessage),
    Error(status::ErrorMessage),

    ServerInfo(connection::ServerInfoMessage),

    DeviceList(device::information::DeviceListMessage),
    ScanningFinished(device::ScanningFinishedMessage),

    InputReading(device::input::InputReadingMessage),
}

impl MessageRequest {
    pub fn request_server_info(id: u32, client_name: impl Into<String>) -> Self {
        let data = connection::RequestServerInfoMessage {
            id,
            client_name: client_name.into(),
            protocol_version_major: CURRENT_SUPPORTED_VERSION_MAJOR,
            protocol_version_minor: CURRENT_SUPPORTED_VERSION_MINOR,
        };

        Self::RequestServerInfo(data)
    }
}
