use serde::{Deserialize, Serialize};

use super::{GenericThing, MetricsRecord, Statistics};

#[derive(Debug, Deserialize, Clone)]
pub struct VolumeRecords {
    pub records: Vec<Volume>,
    pub num_records: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Volume {
    pub uuid: String,
    pub name: String,
    pub size: u128,
    pub style: Option<VolumeStyle>,
    pub svm: GenericThing,
    pub aggregates: Vec<GenericThing>,
    pub encryption: VolumeEncryption,
    pub space: VolumeSpace,
    pub statistics: Option<Statistics>,
    pub clone: VolumeClone,
    pub autosize: VolumeAutosize,
    pub files: VolumeFiles,
    pub movement: Option<VolumeMovement>,
    pub efficiency: Option<VolumeEfficiency>,
    pub constituents: Option<Vec<VolumeConstituents>>,
}

impl Volume {
    pub fn percent_avail(&self) -> u128 {
        self.space.available * 100 / self.size
    }
    pub fn percent_used(&self) -> u128 {
        self.space.used * 100 / self.size
    }
    pub fn files_percent_used(&self) -> u128 {
        self.files.used * 100 / self.files.maximum
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VolumeSpace {
    pub available: u128,
    pub block_storage_inactive_user_data: Option<u128>,
    pub capacity_tier_footprint: Option<u128>,
    pub footprint: Option<u128>,
    //pub logical_space	{...}
    pub metadata: Option<u128>,
    pub over_provisioned: u128,
    pub size: u128,
    pub snapshot: VolumeSpaceSnapshot,
    pub used: u128,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VolumeConstituents {
    pub aggregates: Option<GenericThing>,
    pub movement: Option<VolumeMovement>,
    pub name : String,
    pub space : Option<VolumeSpace>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VolumeEfficiency {
    pub compaction: VolumeEfficencyCompaction,
    pub compression: VolumeEfficencyCompression,
    pub cross_volume_dedupe: VolumeEfficencyDedupe,
    pub dedupe: VolumeEfficencyDedupe,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VolumeEncryption {
    pub enabled: bool,
    pub state: Option<VolumeEncrpytionState>,
    #[serde(rename = "type")]
    pub typ: Option<VolumeEncrpytionType>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VolumeSpaceSnapshot {
    pub autodelete_enabled: bool,
    pub reserve_percent: u8,
    pub used: u128,
}

#[derive(Debug, Deserialize)]
pub struct VolumeMetricRecords {
    pub records: Vec<MetricsRecord>,
    pub num_records: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VolumeClone {
    pub is_flexclone: bool,
    pub parent_volume: Option<GenericThing>,
    pub parent_svm: Option<GenericThing>,
    pub split_complete_percent: Option<u128>,
    pub split_estimate: Option<u128>,
    pub split_initiated: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VolumeAutosize {
    pub grow_threshold: u128,
    pub maximum: u128,
    pub minimum: u128,
    pub mode: String,
    pub shrink_threshold: u128,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VolumeFiles {
    pub maximum: u128,
    pub used: u128,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VolumeMovement {
    pub state: MovementState,
    pub cutover_window: u32,
    pub destination_aggregate: GenericThing,
    pub percent_complete: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum VolumeEncrpytionState {
    Encrypted,
    Encrypting,
    Partial,
    Rekeying,
    Unencrypted,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum VolumeStyle {
    Flexvol,
    Flexgroup,
    FlexgroupConstituent,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum VolumeEfficencyCompaction {
    Inline, None, Mixed,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum VolumeEfficencyCompression {
    Inline, Background, Both, None, Mixed
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum VolumeEfficencyDedupe {
    Inline, Background, Both, None, Mixed
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum VolumeEncrpytionType {
    None,
    Volume,
    Aggregate,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum MovementState {
    Aborted,
    Cutover,
    CutoverWait,
    CutoverPending,
    Failed,
    Paused,
    Queued,
    Replicating,
    Success,
}

#[derive(Debug, Serialize)]
pub struct VolumeResizeBody {
    pub size: u64,
}

#[derive(Debug, Serialize)]
pub struct VolumeMoveBody {
    pub movement: VolumeMoveMovement,
}

#[derive(Debug, Serialize)]
pub struct VolumeMoveMovement {
    pub cutover_window: u32,
    pub destination_aggregate: VolumeMoveAggregate,
}

#[derive(Debug, Serialize)]
pub struct VolumeMoveAggregate {
    pub uuid: String,
}
