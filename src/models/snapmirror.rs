use serde::{Deserialize, Serialize};

use super::NameOnly;

#[derive(Debug, Deserialize)]
pub struct SnapmirrorRelationshipRecords {
    pub records: Vec<SnapmirrorRelationship>,
    pub num_records: u32,
}

#[derive(Debug, Deserialize)]
pub struct SnapmirrorRelationship {
    pub uuid: String,
    pub destination: SnapmirrorEndpoint,
    pub source: SnapmirrorEndpoint,
    pub state: SnapmirrorRelationshipState,
    pub transfer: Option<TransferState>,
    pub healthy: bool,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename_all = "snake_case")]
pub enum SnapmirrorRelationshipState {
    BrokenOff,
    Paused,
    Snapmirrored,
    Uninitialized,
    InSync,
    OutOfSync,
    Synchronizing,
}

#[derive(Debug, Deserialize)]
pub struct SnapmirrorEndpoint {
    pub path: String,
    pub svm: NameOnly,
}

#[derive(Debug, Deserialize)]
pub struct TransferState {
    pub state: TransferStateState,
    pub uuid: String,
    pub bytes_transferred: u128,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferStateState {
    Aborted,
    Failed,
    HardAborted,
    Queued,
    Success,
    Transferring,
}
