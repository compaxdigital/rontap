use std::fmt::Display;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::GenericThing;

#[derive(Debug, Deserialize)]
pub struct ShelfRecords {
    pub records: Vec<Shelf>,
    pub num_records: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Shelf {
    pub bays: Vec<Bay>,
    pub connection_type: String,
    pub disk_count: u32,
    pub drawers: Option<Vec<Drawer>>,
    pub id: String,
    pub uid: String,
    pub model: String,
    pub name: String,
    pub paths: Vec<Path>,
    pub ports: Option<Vec<Port>>,
    pub state: ShelfState,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ShelfState {
    Unknown,
    Ok,
    Error,
}

impl Display for Shelf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Shelf ")?;
        write!(f, "{:10} |", self.name)?;
        for b in &self.bays {
            if !b.has_disk {
                f.write_str(".")?;
            } else {
                let s = match b.state {
                    DiskState::Error => "E",
                    DiskState::Ok => "D",
                    DiskState::Unknown => "U",
                };
                f.write_str(s)?;
            }
        }
        f.write_str("|")?;

        let s: String = self.paths.iter().map(|p| p.node.name.clone()).join(", ");

        write!(f, " {{{}}}", s)?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bay {
    pub has_disk: bool,
    pub id: u64,
    pub state: DiskState,
    #[serde(rename = "type")]
    pub typ: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiskState {
    Unknown,
    Error,
    Ok,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Drawer {
    pub closed: bool,
    pub disk_count: u64,
    pub error: String,
    pub id: u64,
    pub part_number: String,
    pub serial_number: String,
    pub state: DrawerState,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DrawerState {
    Ok,
    Error,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Path {
    pub name: String,
    pub node: GenericThing,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Port {
    pub cable: Option<Cable>,
    pub designator: String,
    pub id: u64,
    pub internal: bool,
    pub mac_address: Option<String>,
    pub module_id: String,
    pub state: PortState,
    pub wwn: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PortState {
    Connected,
    Disconnected,
    Error,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Cable {
    pub identifier: String,
    pub length: Option<String>,
    pub part_number: Option<String>,
    pub serial_number: Option<String>,
}
