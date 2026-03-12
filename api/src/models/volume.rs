use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Volume {
    pub name: String,
    pub driver: String,
    pub mountpoint: String,
    pub status: Option<HashMap<String, serde_json::Value>>,
    pub labels: HashMap<String, String>,
    pub scope: String,
    pub cluster_volume: Option<ClusterVolume>,
    pub options: HashMap<String, String>,
    pub usage_data: Option<VolumeUsageData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeUsageData {
    pub size: i64,
    pub ref_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClusterVolume {
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub version: Option<crate::models::swarm::ObjectVersion>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub spec: Option<ClusterVolumeSpec>,
    pub info: Option<ClusterVolumeInfo>,
    pub publish_status: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClusterVolumeSpec {
    pub group: Option<String>,
    pub access_mode: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClusterVolumeInfo {
    pub capacity_bytes: Option<i64>,
    pub volume_context: Option<HashMap<String, String>>,
    pub volume_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeListResponse {
    pub volumes: Vec<Volume>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeCreateRequest {
    pub name: Option<String>,
    pub driver: Option<String>,
    pub driver_opts: Option<HashMap<String, String>>,
    pub labels: Option<HashMap<String, String>>,
    pub cluster_volume_spec: Option<ClusterVolumeSpec>,
}
