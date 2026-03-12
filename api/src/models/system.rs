use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemInfo {
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub containers: Option<i32>,
    pub containers_running: Option<i32>,
    pub containers_paused: Option<i32>,
    pub containers_stopped: Option<i32>,
    pub images: Option<i32>,
    pub driver: Option<String>,
    pub driver_status: Option<Vec<Vec<String>>>,
    pub docker_root_dir: Option<String>,
    pub system_status: Option<Vec<Vec<String>>>,
    pub plugins: Option<PluginsInfo>,
    pub memory_limit: Option<bool>,
    pub swap_limit: Option<bool>,
    pub kernel_memory: Option<bool>,
    pub kernel_memory_tcp: Option<bool>,
    pub cpu_cfs_period: Option<bool>,
    pub cpu_cfs_quota: Option<bool>,
    pub cpu_shares: Option<bool>,
    pub cpu_set: Option<bool>,
    pub pids_limit: Option<bool>,
    pub oom_kill_disable: Option<bool>,
    pub ipv4_forwarding: Option<bool>,
    pub bridge_nf_iptables: Option<bool>,
    pub bridge_nf_ip6tables: Option<bool>,
    pub debug: Option<bool>,
    pub n_events_listener: Option<i32>,
    pub kernel_version: Option<String>,
    pub operating_system: Option<String>,
    pub os_type: Option<String>,
    pub architecture: Option<String>,
    pub n_cpu: Option<i32>,
    pub mem_total: Option<i64>,
    pub index_server_address: Option<String>,
    pub registry_config: Option<serde_json::Value>,
    pub generic_resources: Option<Vec<serde_json::Value>>,
    pub http_proxy: Option<String>,
    pub https_proxy: Option<String>,
    pub no_proxy: Option<String>,
    pub name: Option<String>,
    pub labels: Option<Vec<String>>,
    pub experimental_build: Option<bool>,
    pub server_version: Option<String>,
    pub cluster_store: Option<String>,
    pub cluster_advertise: Option<String>,
    pub runtimes: Option<HashMap<String, Runtime>>,
    pub default_runtime: Option<String>,
    pub swarm: Option<SwarmInfo>,
    pub isolation: Option<String>,
    pub init_binary: Option<String>,
    pub containerd_commit: Option<Commit>,
    pub runc_commit: Option<Commit>,
    pub init_commit: Option<Commit>,
    pub security_options: Option<Vec<String>>,
    pub product_license: Option<String>,
    pub default_address_pools: Option<Vec<serde_json::Value>>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PluginsInfo {
    pub volume: Option<Vec<String>>,
    pub network: Option<Vec<String>>,
    pub authorization: Option<Vec<String>>,
    pub log: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Runtime {
    pub path: Option<String>,
    pub runtime_args: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Commit {
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub expected: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmInfo {
    pub node_id: Option<String>,
    pub node_addr: Option<String>,
    pub local_node_state: Option<String>,
    pub control_available: Option<bool>,
    pub error: Option<String>,
    pub remote_managers: Option<Vec<serde_json::Value>>,
    pub nodes: Option<i32>,
    pub managers: Option<i32>,
    pub cluster: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SystemVersion {
    pub platform: Option<serde_json::Value>,
    pub components: Option<Vec<ComponentVersion>>,
    pub version: Option<String>,
    pub api_version: Option<String>,
    pub min_api_version: Option<String>,
    pub git_commit: Option<String>,
    pub go_version: Option<String>,
    pub os: Option<String>,
    pub arch: Option<String>,
    pub kernel_version: Option<String>,
    pub experimental: Option<bool>,
    pub build_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ComponentVersion {
    pub name: String,
    pub version: String,
    pub details: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventMessage {
    #[serde(rename = "Type")]
    pub event_type: Option<String>,
    pub action: Option<String>,
    pub actor: Option<EventActor>,
    pub scope: Option<String>,
    pub time: Option<i64>,
    pub time_nano: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EventActor {
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub attributes: Option<HashMap<String, String>>,
}
