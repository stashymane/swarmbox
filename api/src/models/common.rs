use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImageHistoryResponseItem {
    #[serde(rename = "Id")]
    pub id: String,
    pub created: i64,
    pub created_by: String,
    pub tags: Vec<String>,
    pub size: i64,
    pub comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PortSummary {
    #[serde(rename = "IP")]
    pub ip: Option<String>,
    pub private_port: u16,
    pub public_port: Option<u16>,
    #[serde(rename = "Type")]
    pub port_type: PortType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PortType {
    Tcp,
    Udp,
    Sctp,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MountType {
    Bind,
    Cluster,
    Image,
    Npipe,
    Tmpfs,
    Volume,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MountPoint {
    #[serde(rename = "Type")]
    pub mount_type: Option<MountType>,
    pub name: Option<String>,
    pub source: Option<String>,
    pub destination: Option<String>,
    pub driver: Option<String>,
    pub mode: Option<String>,
    pub rw: Option<bool>,
    pub propagation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceMapping {
    pub path_on_host: Option<String>,
    pub path_in_container: Option<String>,
    pub cgroup_permissions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceRequest {
    pub driver: Option<String>,
    pub count: Option<i64>,
    pub device_i_ds: Option<Vec<String>>,
    pub capabilities: Option<Vec<Vec<String>>>,
    pub options: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ThrottleDevice {
    pub path: Option<String>,
    pub rate: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Mount {
    pub target: Option<String>,
    pub source: Option<String>,
    #[serde(rename = "Type")]
    pub mount_type: Option<MountType>,
    pub read_only: Option<bool>,
    pub consistency: Option<String>,
    pub bind_options: Option<BindOptions>,
    pub volume_options: Option<VolumeOptions>,
    pub tmpfs_options: Option<TmpfsOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BindOptions {
    pub propagation: Option<String>,
    pub non_recursive: Option<bool>,
    pub create_mountpoint: Option<bool>,
    pub read_only_non_recursive: Option<bool>,
    pub read_only_force_if_recursive: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct VolumeOptions {
    pub no_copy: Option<bool>,
    pub labels: Option<std::collections::HashMap<String, String>>,
    pub driver_config: Option<DriverConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DriverConfig {
    pub name: Option<String>,
    pub options: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DriverData {
    pub name: Option<String>,
    pub data: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TmpfsOptions {
    pub size_bytes: Option<i64>,
    pub mode: Option<u32>,
}
