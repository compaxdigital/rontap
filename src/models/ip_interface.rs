use serde::{Deserialize, Serialize};

use super::{GenericThing, NameOnly};

#[derive(Debug, Deserialize)]
pub struct IpInterfaceRecords {
    pub records: Vec<IpInterface>,
    pub num_records: u32,
}

#[derive(Debug, Deserialize)]
pub struct IpInterface {
    pub enabled: bool,
    pub ip: IpInfo,
    pub ipspace: GenericThing,
    pub location: IpInterfaceLocation,
    pub name: String,
    pub scope: Scope,
    pub service_policy: GenericThing,
    pub services: Vec<Service>,
    pub state: OperationalState,
    pub svm: Option<GenericThing>,
    pub uuid: String,
    pub vip: bool,
}

#[derive(Debug, Deserialize)]
pub struct IpInterfaceLocation {
    pub auto_revert: bool,
    pub broadcast_domain: Option<GenericThing>,
    pub failover: FailoverScope,
    pub home_node: GenericThing,
    pub home_port: PortInfo,
    pub is_home: bool,
    pub node: GenericThing,
    pub port: PortInfo,
}

#[derive(Debug, Deserialize)]
pub struct PortInfo {
    pub name: String,
    pub node: NameOnly,
    pub uuid: String,
}

#[derive(Debug, Deserialize)]
pub struct IpInfo {
    pub address: String,
    pub family: IpFamily,
    pub netmask: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Service {
    ClusterCore,
    InterclusterCore,
    ManagementCore,
    ManagementAutosupport,
    ManagementBgp,
    ManagementEms,
    ManagementHttps,
    ManagementSsh,
    ManagementPortmap,
    DataCore,
    DataNfs,
    DataCifs,
    DataFlexcache,
    DataIscsi,
    DataS3Server,
    DataFpolicyClient,
    ManagementDnsClient,
    ManagementAdClient,
    ManagementLdapClient,
    ManagementNisClient,
    ManagementNtpClient,
    DataDnsServer,
    ManagementHttp,
    BackupNdmpControl,
    ManagementSnmpServer,
    ManagementNtpServer,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OperationalState {
    Up,
    Down,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    Svm,
    Cluster,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename_all = "snake_case")]
pub enum FailoverScope {
    HomePortOnly,
    Default,
    HomeNodeOnly,
    SfoPartnersOnly,
    BroadcastDomainOnly,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename_all = "snake_case")]
pub enum IpFamily {
    Ipv4,
    Ipv6,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct IpInterfaceAdminState {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct IpInterfaceHomeState {
    pub location: IpInterfaceHomeStateLocation,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct IpInterfaceHomeStateLocation {
    pub is_home: bool,
}
