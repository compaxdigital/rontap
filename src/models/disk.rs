use serde::{Deserialize, Serialize};

use super::GenericThing;

#[derive(Debug, Deserialize)]
pub struct DiskRecords {
    pub records: Vec<Disk>,
    pub num_records: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Disk {
    pub aggregates: Option<Vec<GenericThing>>,
    pub bay: u32,
    pub class: DiskClass,
    pub container_type: ContainerType,
    pub dr_node: Option<GenericThing>,
    pub fips_certified: Option<bool>,
    pub firmware_version: String,
    pub home_node: GenericThing,
    pub model: String,
    pub name: String,
    pub node: GenericThing,
    pub pool: String,
    pub protection_mode: Option<String>,
    pub rated_life_used_percent: Option<u32>,
    pub rpm: Option<u32>,
    pub sector_count: Option<u32>,
    pub self_encrypting: bool,
    pub serial_number: String,
    pub state: DiskState,
    #[serde(rename = "type")]
    pub typ: DiskType,
    pub uid: String,
    pub usable_size: u64,
    pub vendor: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum DiskType {
    Ata,
    Bsas,
    Fcal,
    Fsas,
    Lun,
    Sas,
    Msata,
    Ssd,
    Vmdisk,
    Unknown,
    SsdCap,
    SsdNvm,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum DiskState {
    Broken,
    Copy,
    Maintenance,
    Partner,
    Pending,
    Present,
    Reconstructing,
    Removed,
    Spare,
    Unfail,
    Zeroing,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum DiskClass {
    Unknown,
    Capacity,
    Performance,
    Archive,
    SolidState,
    Array,
    Virtual,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ContainerType {
    Aggregate,
    Broken,
    Foreign,
    Labelmaint,
    Maintenance,
    Shared,
    Spare,
    Unassigned,
    Unknown,
    Unsupported,
    Remote,
    Mediator,
}
