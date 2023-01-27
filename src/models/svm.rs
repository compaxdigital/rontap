use serde::{Deserialize, Serialize};

use super::GenericThing;

#[derive(Debug, Deserialize)]
pub struct SvmRecords {
    pub records: Vec<Svm>,
    pub num_records: u32,
}

#[derive(Debug, Deserialize)]
pub struct Svm {
    pub uuid: String,
    pub name: String,
    pub state: SvmState,
    pub aggregates: Vec<GenericThing>,
    pub cifs: Option<SvmCifs>,
    pub ldap: Option<SvmLdap>,
    pub nfs: Option<SvmNfs>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum SvmState {
    Starting,
    Running,
    Stopping,
    Stopped,
    Deleting,
}

#[derive(Debug, Deserialize)]
pub struct SvmCifs {
    pub enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SvmLdap {
    pub enabled: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SvmNfs {
    pub enabled: Option<bool>,
}
