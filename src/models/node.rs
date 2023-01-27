use serde::{Deserialize, Serialize};

use super::NetworkAddress;

#[derive(Debug, Deserialize)]
pub struct NodeRecords {
    pub records: Vec<Node>,
    pub num_records: u32,
}

#[derive(Debug, Deserialize)]
pub struct Node {
    pub uuid: String,
    pub name: String,
    pub model: String,
    pub state: NodeState,
    pub serial_number: String,
    pub uptime: u64,
    pub service_processor: NodeServiceProcessor,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NodeState {
    Up,
    Booting,
    Down,
    TakenOver,
    WaitingForGiveback,
    Degraded,
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct NodeServiceProcessor {
    pub firmware_version: String,
    pub ipv4_interface: NetworkAddress,
    pub link_status: String,
    pub state: NodeServiceProcessorState,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeServiceProcessorState {
    Online,
    Offline,
    Degraded,
    Rebooting,
    Unknown,
    Updating,
    NodeOffline,
    SpDaemonOffline,
}
