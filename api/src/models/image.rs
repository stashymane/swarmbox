use crate::models::common::ImageHistoryResponseItem;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageSummary {
    #[serde(rename = "Id")]
    pub id: String,
    pub parent_id: String,
    pub repo_tags: Vec<String>,
    pub repo_digests: Vec<String>,
    pub created: i64,
    pub size: i64,
    pub shared_size: i64,
    pub virtual_size: Option<i64>,
    pub labels: HashMap<String, String>,
    pub containers: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageInspect {
    #[serde(rename = "Id")]
    pub id: Option<String>,
    pub repo_tags: Option<Vec<String>>,
    pub repo_digests: Option<Vec<String>>,
    pub parent: Option<String>,
    pub comment: Option<String>,
    pub created: Option<String>,
    pub container: Option<String>,
    pub container_config: Option<crate::models::container::ContainerConfig>,
    pub docker_version: Option<String>,
    pub author: Option<String>,
    pub config: Option<crate::models::container::ContainerConfig>,
    pub architecture: Option<String>,
    pub os: Option<String>,
    #[serde(rename = "OsVersion")]
    pub os_version: Option<String>,
    pub size: Option<i64>,
    pub virtual_size: Option<i64>,
    pub graph_driver: Option<crate::models::common::DriverData>,
    #[serde(rename = "RootFS")]
    pub rootfs: Option<RootFs>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RootFs {
    #[serde(rename = "Type")]
    pub rootfs_type: String,
    pub layers: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageHistory {
    pub items: Vec<ImageHistoryResponseItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AuthConfig {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub serveraddress: Option<String>,
    pub identitytoken: Option<String>,
    pub registrytoken: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageDeleteResponseItem {
    pub untagged: Option<String>,
    pub deleted: Option<String>,
}
