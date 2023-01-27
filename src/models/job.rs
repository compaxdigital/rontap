use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JobRecords {
    pub records: Vec<Job>,
    pub num_records: u32,
}

#[derive(Debug, Deserialize)]
pub struct Job {
    pub uuid: String,
    pub state: JobState,
    pub code: Option<u32>,
    pub description: Option<String>,
    pub message: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JobState {
    Queued,
    Running,
    Paused,
    Success,
    Failure,
}
