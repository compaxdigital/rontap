use serde::{Deserialize, Serialize};

use super::{GenericThing, NameOnly};

#[derive(Debug, Deserialize)]
pub struct PortRecords {
    pub records: Vec<Port>,
    pub num_records: u32,
}

#[derive(Debug, Deserialize)]
pub struct Port {
    pub broadcast_domain: Option<PortBroadcastDomain>,
    pub enabled: bool,
    pub lag: Option<PortLag>,
    pub mac_address: String,
    pub mtu: u32,
    pub name: String,
    pub node: GenericThing,
    pub speed: Option<u32>,
    pub state: PortState,
    #[serde(rename = "type")]
    pub typ: PortType,
    pub uuid: String,
    pub vlan: Option<PortVlan>,
}

#[derive(Debug, Deserialize)]
pub struct PortBroadcastDomain {
    pub ipspace: NameOnly,
    pub name: String,
    pub uuid: String,
}

#[derive(Debug, Deserialize)]
pub struct PortVlan {
    pub base_port: GenericThing,
    pub tag: u32,
}

#[derive(Debug, Deserialize)]
pub struct PortLag {
    pub active_ports: Option<Vec<GenericThing>>,
    pub distribution_policy: PortLagDistributionPolicy,
    pub member_ports: Option<Vec<GenericThing>>,
    pub mode: PortLagMode,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PortLagDistributionPolicy {
    Port,
    Ip,
    Mac,
    Sequential,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PortLagMode {
    MultimodeLacp,
    Multimode,
    Singlemode,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PortState {
    Up,
    Down,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PortType {
    Vlan,
    Physical,
    Lag,
}
