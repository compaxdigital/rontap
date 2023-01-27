use serde::{Deserialize, Serialize};

use super::{MetricsRecord, NameOnly, Statistics};

#[derive(Debug, Deserialize, Clone)]
pub struct AggregateRecords {
    pub records: Vec<Aggregate>,
    pub num_records: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Aggregate {
    pub uuid: String,
    pub name: String,
    pub space: AggregateSpace,
    pub block_storage: AggregateBlockStorage,
    pub statistics: Option<Statistics>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AggregateSpace {
    pub block_storage: BlockStorage,
    pub efficiency: Efficiency,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BlockStorage {
    pub size: u128,
    pub available: u128,
    pub used: u128,
    pub full_threshold_percent: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Efficiency {
    pub savings: u128,
    pub ratio: f32,
    pub logical_used: u128,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AggregateBlockStorage {
    pub primary: AggregatePrimaryBlockStorage,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AggregatePrimaryBlockStorage {
    pub checksum_style: String,
    pub disk_class: String,
    pub disk_count: u32,
    pub disk_type: String,
    pub raid_size: u32,
    pub raid_type: String,
}

#[derive(Debug, Deserialize)]
pub struct AggregateMetricRecords {
    pub records: Vec<MetricsRecord>,
    pub num_records: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PlexRecords {
    pub records: Vec<Plex>,
    pub num_records: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Plex {
    pub aggregate: NameOnly,
    pub name: String,
    pub online: bool,
    pub pool: String,
    pub raid_groups: Vec<RaidGroup>,
    pub resync: Resync,
    pub state: PlexState,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RaidGroup {
    pub cache_tier: bool,
    pub degraded: bool,
    pub disks: Vec<RaidGroupDisk>,
    pub name: String,
    pub recomputing_parity: RecomputingParity,
    pub reconstruct: Reconstruct,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RaidGroupDisk {
    pub disk: Option<NameOnly>,
    pub position: RaidGroupDiskPosition,
    pub state: RaidGroupDiskState,
    #[serde(rename = "type")]
    pub typ: Option<super::disk::DiskType>,
    pub usable_size: u128,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Resync {
    pub active: bool,
    pub level: Option<String>,
    pub percent: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Reconstruct {
    pub active: bool,
    pub percent: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RecomputingParity {
    pub active: bool,
    pub percent: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RaidGroupDiskPosition {
    Data,
    Parity,
    Dparity,
    Tparity,
    Copy,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum RaidGroupDiskState {
    Normal,
    Failed,
    Zeroing,
    Copy,
    Replacing,
    Evacuating,
    Prefail,
    Offline,
    Reconstructing,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum PlexState {
    Normal,
    Failed,
    OutOfDate,
}
