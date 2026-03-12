use crate::models::common::{DeviceMapping, DeviceRequest, Mount, MountPoint, ThrottleDevice};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostConfig {
    pub cpu_shares: Option<i64>,
    pub memory: Option<i64>,
    pub cgroup_parent: Option<String>,
    pub blkio_weight: Option<u16>,
    pub blkio_weight_device: Option<Vec<WeightDevice>>,
    pub blkio_device_read_bps: Option<Vec<ThrottleDevice>>,
    pub blkio_device_write_bps: Option<Vec<ThrottleDevice>>,
    pub blkio_device_read_iops: Option<Vec<ThrottleDevice>>,
    pub blkio_device_write_iops: Option<Vec<ThrottleDevice>>,
    pub cpu_period: Option<i64>,
    pub cpu_quota: Option<i64>,
    pub cpu_realtime_period: Option<i64>,
    pub cpu_realtime_runtime: Option<i64>,
    pub cpuset_cpus: Option<String>,
    pub cpuset_mems: Option<String>,
    pub devices: Option<Vec<DeviceMapping>>,
    pub device_cgroup_rules: Option<Vec<String>>,
    pub device_requests: Option<Vec<DeviceRequest>>,
    pub kernel_memory_tcp: Option<i64>,
    pub memory_reservation: Option<i64>,
    pub memory_swap: Option<i64>,
    pub memory_swappiness: Option<i64>,
    pub nano_cpus: Option<i64>,
    pub oom_kill_disable: Option<bool>,
    pub init: Option<bool>,
    pub pids_limit: Option<i64>,
    pub ulimits: Option<Vec<Ulimit>>,
    pub cpu_count: Option<i64>,
    pub cpu_percent: Option<i64>,
    #[serde(rename = "IOMaximumIOps")]
    pub io_maximum_iops: Option<u64>,
    #[serde(rename = "IOMaximumBandwidth")]
    pub io_maximum_bandwidth: Option<u64>,
    pub binds: Option<Vec<String>>,
    pub container_id_file: Option<String>,
    pub log_config: Option<LogConfig>,
    pub network_mode: Option<String>,
    pub port_bindings: Option<HashMap<String, Vec<PortBinding>>>,
    pub restart_policy: Option<RestartPolicy>,
    pub auto_remove: Option<bool>,
    pub volume_driver: Option<String>,
    pub volumes_from: Option<Vec<String>>,
    pub mounts: Option<Vec<Mount>>,
    pub console_size: Option<Vec<u32>>,
    pub annotations: Option<HashMap<String, String>>,
    pub cap_add: Option<Vec<String>>,
    pub cap_drop: Option<Vec<String>>,
    pub dns: Option<Vec<String>>,
    pub dns_options: Option<Vec<String>>,
    pub dns_search: Option<Vec<String>>,
    pub extra_hosts: Option<Vec<String>>,
    pub group_add: Option<Vec<String>>,
    pub ipc_mode: Option<String>,
    pub cgroup: Option<String>,
    pub links: Option<Vec<String>>,
    pub oom_score_adj: Option<i64>,
    pub pid_mode: Option<String>,
    pub privileged: Option<bool>,
    pub publish_all_ports: Option<bool>,
    pub readonly_rootfs: Option<bool>,
    pub security_opt: Option<Vec<String>>,
    pub storage_opt: Option<HashMap<String, String>>,
    pub tmpfs: Option<HashMap<String, String>>,
    pub uts_mode: Option<String>,
    pub userns_mode: Option<String>,
    pub shm_size: Option<i64>,
    pub sysctls: Option<HashMap<String, String>>,
    pub runtime: Option<String>,
    pub isolation: Option<Isolation>,
    pub masked_paths: Option<Vec<String>>,
    pub readonly_paths: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct WeightDevice {
    pub path: Option<String>,
    pub weight: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ulimit {
    pub name: Option<String>,
    pub soft: Option<i64>,
    pub hard: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LogConfig {
    #[serde(rename = "Type")]
    pub log_type: Option<LogType>,
    pub config: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LogType {
    #[serde(rename = "json-file")]
    JsonFile,
    Syslog,
    Journald,
    Gelf,
    Fluentd,
    Awslogs,
    Splunk,
    Etwlogs,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PortBinding {
    #[serde(rename = "HostIp")]
    pub host_ip: Option<String>,
    pub host_port: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RestartPolicy {
    pub name: Option<RestartPolicyName>,
    pub maximum_retry_count: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RestartPolicyName {
    #[serde(rename = "")]
    Empty,
    No,
    Always,
    #[serde(rename = "on-failure")]
    OnFailure,
    #[serde(rename = "unless-stopped")]
    UnlessStopped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Isolation {
    Default,
    Process,
    HyperV,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerConfig {
    pub hostname: Option<String>,
    pub domainname: Option<String>,
    pub user: Option<String>,
    pub attach_stdin: Option<bool>,
    pub attach_stdout: Option<bool>,
    pub attach_stderr: Option<bool>,
    pub exposed_ports: Option<HashMap<String, HashMap<(), ()>>>,
    pub tty: Option<bool>,
    pub open_stdin: Option<bool>,
    pub stdin_once: Option<bool>,
    pub env: Option<Vec<String>>,
    pub cmd: Option<Vec<String>>,
    pub healthcheck: Option<HealthConfig>,
    pub args_escaped: Option<bool>,
    pub image: Option<String>,
    pub volumes: Option<HashMap<String, HashMap<(), ()>>>,
    pub working_dir: Option<String>,
    pub entrypoint: Option<Vec<String>>,
    pub network_disabled: Option<bool>,
    pub mac_address: Option<String>,
    pub on_build: Option<Vec<String>>,
    pub labels: Option<HashMap<String, String>>,
    pub stop_signal: Option<String>,
    pub stop_timeout: Option<i64>,
    pub shell: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HealthConfig {
    pub test: Option<Vec<String>>,
    pub interval: Option<i64>,
    pub timeout: Option<i64>,
    pub retries: Option<i64>,
    pub start_period: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerSummary {
    #[serde(rename = "Id")]
    pub id: Option<String>,
    pub names: Option<Vec<String>>,
    pub image: Option<String>,
    #[serde(rename = "ImageID")]
    pub image_id: Option<String>,
    pub command: Option<String>,
    pub created: Option<i64>,
    pub ports: Option<Vec<crate::models::common::PortSummary>>,
    pub size_rw: Option<i64>,
    pub size_root_fs: Option<i64>,
    pub labels: Option<HashMap<String, String>>,
    pub state: Option<String>,
    pub status: Option<String>,
    pub host_config: Option<HostConfigSummary>,
    pub network_settings: Option<NetworkSettingsSummary>,
    pub mounts: Option<Vec<MountPoint>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HostConfigSummary {
    pub network_mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkSettingsSummary {
    pub networks: Option<HashMap<String, crate::models::network::EndpointSettings>>,
}
