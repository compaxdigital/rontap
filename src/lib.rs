use itertools::Itertools;
use reqwest::{ClientBuilder, Url};

use models::{
    aggregate::{AggregateMetricRecords, AggregateRecords, PlexRecords},
    disk::DiskRecords,
    ip_interface::{
        IpInterfaceAdminState, IpInterfaceHomeState, IpInterfaceHomeStateLocation,
        IpInterfaceRecords,
    },
    job::JobRecords,
    nfs::NfsClientRecords,
    node::NodeRecords,
    shelf::ShelfRecords,
    snapmirror::SnapmirrorRelationshipRecords,
    snapshot::SnapshotRecords,
    svm::SvmRecords,
    volume::{
        Volume, VolumeMetricRecords, VolumeMoveAggregate, VolumeMoveBody, VolumeMoveMovement,
        VolumeRecords, VolumeResizeBody,
    },
    AggregatedMetrics, Cluster, Duration, Metrics, MetricsRecord, OntapApiError, Status, Version,
};
use tracing::instrument;

pub mod models;

pub enum ApiVersion {
    V9,
    V9_7,
    V9_8,
}

impl TryFrom<Version> for ApiVersion {
    type Error = OntapApiError;

    fn try_from(value: Version) -> Result<Self, Self::Error> {
        match value {
            Version {
                generation: 9,
                major,
                ..
            } if major < 7 => Ok(Self::V9),
            Version {
                generation: 9,
                major: 7,
                ..
            } => Ok(Self::V9_7),
            Version {
                generation: 9,
                major,
                ..
            } if major >= 8 => Ok(Self::V9_8),
            _ => Err(OntapApiError::UnsupportedApiVersion),
        }
    }
}

impl ApiVersion {
    pub fn volume_fields(&self) -> &str {
        match self {
            _ => "size,svm,aggregates,space,clone,autosize,files,movement",
        }
    }
}

pub struct OntapConnectionParams {
    pub url: Url,
    pub username: String,
    pub password: String,
}

impl OntapConnectionParams {
    pub async fn connect(self) -> Result<OntapClient, OntapApiError> {
        let reqwest_client = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()?;

        let url = format!("{}/cluster", self.url);
        let res = reqwest_client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header("accept", "application/json")
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCode(res.status().as_u16()));
        }
        let cluster: Cluster = res.json().await?;
        let api_version = ApiVersion::try_from(cluster.version)?;

        Ok(OntapClient {
            url: self.url,
            username: self.username,
            password: self.password,
            client: reqwest_client,
            api_version,
        })
    }
}

pub struct OntapClient {
    url: Url,
    username: String,
    password: String,
    client: reqwest::Client,
    api_version: ApiVersion,
}

impl OntapClient {
    pub async fn get_volumes(&self) -> Result<VolumeRecords, OntapApiError> {
        let url = format!("{}/storage/volumes", self.url);
        let res = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header("accept", "application/json")
            .query(&[("fields", self.api_version.volume_fields())])
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCode(res.status().as_u16()));
        }
        Ok(res.json().await?)
    }

    pub async fn get_ip_interfaces(&self) -> Result<IpInterfaceRecords, OntapApiError> {
        let url = format!("{}/network/ip/interfaces", self.url);
        let res = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header("accept", "application/json")
            .query(&[(
                "fields",
                "enabled,ip,ipspace,location,name,scope,service_policy,services,state,svm,uuid,vip",
            )])
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCode(res.status().as_u16()));
        }
        Ok(res.json().await?)
    }

    pub async fn ip_interface_set_admin_status(
        &self,
        uuid: &str,
        enabled: bool,
    ) -> Result<(), OntapApiError> {
        let url = format!("{}/network/ip/interfaces/{uuid}", self.url);
        let res = self
            .client
            .patch(url)
            .json(&IpInterfaceAdminState { enabled })
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCodeWithError(
                res.status().as_u16(),
                res.text().await?,
            ));
        }
        Ok(())
    }

    pub async fn ip_interface_revert(&self, uuid: &str) -> Result<(), OntapApiError> {
        let url = format!("{}/network/ip/interfaces/{uuid}", self.url);
        let res = self
            .client
            .patch(url)
            .json(&IpInterfaceHomeState {
                location: IpInterfaceHomeStateLocation { is_home: true },
            })
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCodeWithError(
                res.status().as_u16(),
                res.text().await?,
            ));
        }
        Ok(())
    }

    pub async fn get_shelves(&self) -> Result<ShelfRecords, OntapApiError> {
        let url = format!("{}/storage/shelves", self.url);
        let res = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header("accept", "application/json")
            .query(&[(
                "fields",
                "bays,connection_type,disk_count,drawers,id,uid,model,name,paths,ports,state",
            )])
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCode(res.status().as_u16()));
        }
        Ok(res.json().await?)
    }

    pub async fn get_disks(&self) -> Result<DiskRecords, OntapApiError> {
        let url = format!("{}/storage/disks", self.url);
        let res = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header("accept", "application/json")
            .query(&[(
                "fields",
                "aggregates,bay,class,container_type,dr_node,fips_certified,firmware_version,home_node,model,name,node,pool,protection_mode,rated_life_used_percent,rpm,self_encrypting,serial_number,state,type,uid,usable_size,vendor",
            )])
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCodeWithError(
                res.status().as_u16(),
                res.text().await?,
            ));
        }
        Ok(res.json().await?)
    }

    pub async fn resize_volume(&self, uuid: &str, new_size: u64) -> Result<(), OntapApiError> {
        let url = format!("{url}/storage/volumes/{uuid}", url = self.url, uuid = uuid);
        let res = self
            .client
            .patch(url)
            .json(&VolumeResizeBody { size: new_size })
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCode(res.status().as_u16()));
        }
        Ok(())
    }

    pub async fn move_volume(
        &self,
        uuid: &str,
        destination_aggregate_uuid: &str,
        cutover_window: u32,
    ) -> Result<(), OntapApiError> {
        let url = format!("{url}/storage/volumes/{uuid}", url = self.url, uuid = uuid);
        let body = VolumeMoveBody {
            movement: VolumeMoveMovement {
                cutover_window,
                destination_aggregate: VolumeMoveAggregate {
                    uuid: destination_aggregate_uuid.into(),
                },
            },
        };
        let res = self
            .client
            .patch(url)
            .json(&body)
            .basic_auth(&self.username, Some(&self.password))
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCode(res.status().as_u16()));
        }
        Ok(())
    }

    pub async fn get_volume(&self, uuid: &str) -> Result<Volume, OntapApiError> {
        let url = format!("{}/storage/volumes/{}", self.url, uuid);

        let res = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header("accept", "application/json")
            .query(&[("fields", "size,svm,aggregates,space")])
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCode(res.status().as_u16()));
        }
        Ok(res.json().await?)
    }

    #[instrument(skip(self))]
    pub async fn get_volume_metrics(
        &self,
        uuid: &str,
    ) -> Result<VolumeMetricRecords, OntapApiError> {
        let url = format!("{}/storage/volumes/{}/metrics", self.url, uuid);

        let res = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header("accept", "application/json")
            .query(&[
                ("fields", "duration,iops,throughput,status,timestamp"),
                ("duration", "PT15S"),
            ])
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCode(res.status().as_u16()));
        }
        Ok(res.json().await?)
    }

    pub async fn get_volume_snapshots(&self, uuid: &str) -> Result<SnapshotRecords, OntapApiError> {
        let url = format!("{}/storage/volumes/{}/snapshots", self.url, uuid);
        let res = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header("accept", "application/json")
            .query(&[("fields", "uuid,name,owners,state")])
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCodeWithError(
                res.status().as_u16(),
                res.text().await.unwrap(),
            ));
        }
        Ok(res.json().await?)
    }

    pub async fn get_aggregates(&self) -> Result<AggregateRecords, OntapApiError> {
        let url = format!("{}/storage/aggregates", self.url);
        let res = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header("accept", "application/json")
            .query(&[("fields", "space,block_storage")])
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCode(res.status().as_u16()));
        }
        Ok(res.json().await?)
    }

    pub async fn get_nfs_clients(&self) -> Result<NfsClientRecords, OntapApiError> {
        let url = format!("{}/protocols/nfs/connected-clients", self.url);
        let res = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header("accept", "application/json")
            .query(&[("fields", "client_ip,idle_duration,local_request_count,node,protocol,remote_request_count,server_ip,svm,volume")])
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCode(res.status().as_u16()));
        }
        Ok(res.json().await?)
    }

    pub async fn get_plexes(&self, aggregate_uuid: &str) -> Result<PlexRecords, OntapApiError> {
        let url = format!("{}/storage/aggregates/{aggregate_uuid}/plexes", self.url);
        let res = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header("accept", "application/json")
            .query(&[(
                "fields",
                "aggregate,name,online,pool,raid_groups,resync,state",
            )])
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCode(res.status().as_u16()));
        }
        Ok(res.json().await?)
    }

    #[instrument(skip(self))]
    pub async fn get_aggregate_metrics(
        &self,
        uuid: &str,
    ) -> Result<AggregateMetricRecords, OntapApiError> {
        let url = format!("{}/storage/aggregates/{}/metrics", self.url, uuid);

        let res = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header("accept", "application/json")
            .query(&[
                ("fields", "duration,iops,throughput,status,timestamp"),
                ("duration", "PT15S"),
            ])
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCode(res.status().as_u16()));
        }
        Ok(res.json().await?)
    }

    pub async fn get_jobs(&self) -> Result<JobRecords, OntapApiError> {
        let url = format!("{}/cluster/jobs", self.url);
        let res = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header("accept", "application/json")
            .query(&[(
                "fields",
                "state,message,description,code,start_time,end_time",
            )])
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCode(res.status().as_u16()));
        }
        Ok(res.json().await?)
    }

    pub async fn get_svms(&self) -> Result<SvmRecords, OntapApiError> {
        let url = format!("{}/svm/svms", self.url);
        let res = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header("accept", "application/json")
            .query(&[("fields", "aggregates,state,cifs,ldap,nfs")])
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCode(res.status().as_u16()));
        }
        Ok(res.json().await?)
    }

    pub async fn get_nodes(&self) -> Result<NodeRecords, OntapApiError> {
        let url = format!("{}/cluster/nodes", self.url);
        let res = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header("accept", "application/json")
            .query(&[(
                "fields",
                "state,model,serial_number,uptime,service_processor",
            )])
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCode(res.status().as_u16()));
        }
        Ok(res.json().await?)
    }

    pub async fn get_snapmirror_relationships(
        &self,
    ) -> Result<SnapmirrorRelationshipRecords, OntapApiError> {
        let url = format!("{}/snapmirror/relationships", self.url);
        let res = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header("accept", "application/json")
            .query(&[("fields", "source,destination,state,transfer,healthy")])
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCode(res.status().as_u16()));
        }
        Ok(res.json().await?)
    }

    pub async fn get_cluster(&self) -> Result<Cluster, OntapApiError> {
        let url = format!("{}/cluster", self.url);
        let res = self
            .client
            .get(url)
            .basic_auth(&self.username, Some(&self.password))
            .header("accept", "application/json")
            .send()
            .await?;
        if !res.status().is_success() {
            return Err(OntapApiError::HttpStatusCode(res.status().as_u16()));
        }
        Ok(res.json().await?)
    }
}

pub fn aggregate_metrics(metrics: Vec<MetricsRecord>) -> Vec<AggregatedMetrics> {
    metrics
        .into_iter()
        .filter(|r| r.status == Status::Ok)
        .group_by(|r| r.duration)
        .into_iter()
        .map(|(key, group)| {
            group.into_iter().fold(
                AggregatedMetrics {
                    duration: key,
                    iops: Metrics {
                        read: 0,
                        write: 0,
                        total: 0,
                        other: 0,
                    },
                    throughput: Metrics {
                        read: 0,
                        write: 0,
                        total: 0,
                        other: 0,
                    },
                    count: 0,
                },
                |acc, x| {
                    acc + AggregatedMetrics {
                        duration: key,
                        iops: x.iops,
                        throughput: x.throughput,
                        count: 1,
                    }
                },
            )
        })
        .collect::<Vec<_>>()
}

pub fn aggregate_metrics_with_duration(
    metrics: Vec<MetricsRecord>,
    duration: Duration,
) -> Option<AggregatedMetrics> {
    metrics
        .into_iter()
        .filter(|r| r.duration == duration)
        .filter(|r| r.status == Status::Ok)
        .group_by(|r| r.duration)
        .into_iter()
        .map(|(key, group)| {
            group.into_iter().fold(
                AggregatedMetrics {
                    duration: key,
                    iops: Metrics {
                        read: 0,
                        write: 0,
                        total: 0,
                        other: 0,
                    },
                    throughput: Metrics {
                        read: 0,
                        write: 0,
                        total: 0,
                        other: 0,
                    },
                    count: 0,
                },
                |acc, x| {
                    acc + AggregatedMetrics {
                        duration: key,
                        iops: x.iops,
                        throughput: x.throughput,
                        count: 1,
                    }
                },
            )
        })
        .find(|r| r.duration == duration)
}

#[cfg(test)]
mod test {
    use chrono::Utc;

    use crate::models::{AggregatedMetrics, Duration, Metrics, MetricsRecord, Status};

    #[test]
    fn aggregate_metrics() {
        let expected = AggregatedMetrics {
            duration: Duration::P1D,
            iops: Metrics {
                read: 2,
                write: 2,
                total: 2,
                other: 2,
            },
            throughput: Metrics {
                read: 2,
                write: 2,
                total: 2,
                other: 2,
            },
            count: 2,
        };

        let data = vec![
            MetricsRecord {
                duration: Duration::P1D,
                iops: Metrics {
                    read: 1,
                    write: 1,
                    total: 1,
                    other: 1,
                },
                throughput: Metrics {
                    read: 1,
                    write: 1,
                    total: 1,
                    other: 1,
                },
                status: Status::Ok,
                timestamp: Utc::now(),
            },
            MetricsRecord {
                duration: Duration::P1D,
                iops: Metrics {
                    read: 1,
                    write: 1,
                    total: 1,
                    other: 1,
                },
                throughput: Metrics {
                    read: 1,
                    write: 1,
                    total: 1,
                    other: 1,
                },
                status: Status::Error,
                timestamp: Utc::now(),
            },
            MetricsRecord {
                duration: Duration::P1D,
                iops: Metrics {
                    read: 1,
                    write: 1,
                    total: 1,
                    other: 1,
                },
                throughput: Metrics {
                    read: 1,
                    write: 1,
                    total: 1,
                    other: 1,
                },
                status: Status::Ok,
                timestamp: Utc::now(),
            },
        ];

        let actual = super::aggregate_metrics(data.clone());
        assert_eq!(vec![expected.clone()], actual);
        assert_eq!(
            Metrics {
                read: 1,
                write: 1,
                total: 1,
                other: 1,
            },
            actual.get(0).unwrap().avg_iops()
        );
        assert_eq!(
            Metrics {
                read: 1,
                write: 1,
                total: 1,
                other: 1,
            },
            actual.get(0).unwrap().avg_throughput()
        );
        assert_eq!(
            Some(expected.clone()),
            super::aggregate_metrics_with_duration(data.clone(), Duration::P1D)
        );
    }
}
