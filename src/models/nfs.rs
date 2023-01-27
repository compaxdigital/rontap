use serde::{Deserialize, Serialize};

use super::{GenericThing, NameOnly};

#[derive(Debug, Deserialize)]
pub struct NfsClientRecords {
    pub records: Vec<NfsClient>,
    pub num_records: u32,
}

#[derive(Debug, Deserialize)]
pub struct NfsClient {
    pub client_ip: String,
    pub export_policy: Option<NameOnly>,
    pub idle_duration: String,
    pub local_request_count: u64,
    pub node: GenericThing,
    pub protocol: NfsProtocol,
    pub remote_request_count: u64,
    pub server_ip: String,
    pub svm: GenericThing,
    pub volume: GenericThing,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename_all = "snake_case")]
pub enum NfsProtocol {
    Nfs,
    Nfs3,
    Nfs4,
    #[serde(rename = "nfs4.1")]
    Nfs41,
}
