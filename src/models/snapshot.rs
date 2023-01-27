use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct SnapshotRecords {
    pub records: Vec<Snapshot>,
    pub num_records: u32,
}

#[derive(Debug, Deserialize)]
pub struct Snapshot {
    pub uuid: String,
    pub name: String,
    pub owners: Option<Vec<SnapshotOwner>>,
    pub size: Option<u128>,
    pub state: Option<SnapshotState>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotOwner {
    Unknown,
    Snapmirror,
    SnapmirrorDependent,
    SyncMirror,
    VolumeClone,
    VolumeCloneDependent,
    SnapRestore,
    SnapRestoreDependent,
    Dump,
    CifsShare,
    VolumeCopy,
    Ndmp,
    WormVolume,
    SisClone,
    S2cIron,
    LunClone,
    BackupDependent,
    SnaplockDependent,
    FileCloneDependent,
    VolumeMoveDependent,
    SvmdrDependent,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotState {
    Valid,
    Invalid,
    Partial,
}
