use chrono::Utc;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod aggregate;
pub mod disk;
pub mod ip_interface;
pub mod job;
pub mod nfs;
pub mod node;
pub mod port;
pub mod shelf;
pub mod snapmirror;
pub mod snapshot;
pub mod svm;
pub mod volume;
pub mod s3;

#[derive(Error, Debug)]
pub enum OntapApiError {
    #[error("Reqwest error")]
    RequestError(#[from] reqwest::Error),
    #[error("HTTP error status code {0}")]
    HttpStatusCode(u16),
    #[error("HTTP error status code {0} {1}")]
    HttpStatusCodeWithError(u16, String),
    #[error("Unsupported API version")]
    UnsupportedApiVersion,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GenericThing {
    pub uuid: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct NameOnly {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct NetworkAddress {
    pub address: String,
    pub gateway: String,
    pub netmask: String,
}

#[derive(Debug, Deserialize, Copy, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Duration {
    PT15S,
    PT4M,
    PT30M,
    PT2H,
    P1D,
    PT5M,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MetricsRecord {
    pub duration: Duration,
    pub iops: Metrics,
    pub throughput: Metrics,
    pub status: Status,
    pub timestamp: chrono::DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct AggregatedMetrics {
    pub duration: Duration,
    pub iops: Metrics,
    pub throughput: Metrics,
    pub count: u32,
}

impl AggregatedMetrics {
    pub fn avg_iops(&self) -> Metrics {
        Metrics {
            read: self.iops.read / self.count as u128,
            write: self.iops.write / self.count as u128,
            total: self.iops.total / self.count as u128,
            other: self.iops.other / self.count as u128,
        }
    }

    pub fn avg_throughput(&self) -> Metrics {
        Metrics {
            read: self.throughput.read / self.count as u128,
            write: self.throughput.write / self.count as u128,
            total: self.throughput.total / self.count as u128,
            other: self.throughput.other / self.count as u128,
        }
    }
}

impl std::ops::Add for AggregatedMetrics {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            duration: self.duration,
            iops: self.iops + other.iops,
            throughput: self.throughput + other.throughput,
            count: self.count + other.count,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Metrics {
    pub read: u128,
    pub write: u128,
    pub total: u128,
    pub other: u128,
}

impl std::ops::Add for Metrics {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            read: self.read + other.read,
            write: self.write + other.write,
            total: self.total + other.total,
            other: self.other + other.other,
        }
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Ok,
    Error,
    PartialNoData,
    PartialNoUuid,
    PartialNoResponse,
    PartialOtherError,
    NegativeDelta,
    BackfilledData,
    InconsistentDeltaTime,
    InconsistentOldData,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Statistics {
    pub iops_raw: Metrics,
    pub latency_raw: Metrics,
    pub throughput_raw: Metrics,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cluster {
    pub version: Version,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Version {
    pub full: String,
    pub generation: u32,
    pub major: u32,
    pub minor: u32,
}

#[cfg(test)]
mod test {
    use super::{Duration, Metrics};

    #[test]
    fn parse_duration() {
        assert_eq!(Duration::P1D, serde_json::from_str("\"P1D\"").unwrap());
    }

    #[test]
    fn add_metrics() {
        assert_eq!(
            Metrics {
                read: 3,
                write: 3,
                total: 3,
                other: 3,
            },
            Metrics {
                read: 1,
                write: 1,
                total: 1,
                other: 1,
            } + Metrics {
                read: 2,
                write: 2,
                total: 2,
                other: 2,
            }
        )
    }
}
