use serde::{Deserialize, Serialize};

use crate::button::ButtonEvent;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Connected,
    Connecting,
    Disconnected,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum WsRequest {
    ButtonEvent(ButtonEvent),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum WsResponse {
    WsStatusChanged(ConnectionStatus),
}
