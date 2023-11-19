use serde::{Deserialize, Serialize};

use super::{GenericThing, MetricsRecord, Statistics, volume::VolumeEncryption};

#[derive(Debug, Deserialize, Clone)]
pub struct S3BucketRecords {
    pub records: Vec<S3Bucket>,
    pub num_records: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct S3Bucket {
    pub uuid: String,
    pub name: String,
    pub svm: GenericThing,
    pub volume: GenericThing,
    pub size: u128,
    pub encryption: VolumeEncryption,
}
