use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Swarm {
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub version: Option<ObjectVersion>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub spec: Option<SwarmSpec>,
    pub join_tokens: Option<JoinTokens>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ObjectVersion {
    pub index: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SwarmSpec {
    pub name: Option<String>,
    pub labels: Option<HashMap<String, String>>,
    pub orchestration: Option<serde_json::Value>,
    pub raft: Option<serde_json::Value>,
    pub dispatcher: Option<serde_json::Value>,
    pub ca_config: Option<serde_json::Value>,
    pub encryption_config: Option<serde_json::Value>,
    pub task_defaults: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JoinTokens {
    pub worker: String,
    pub manager: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Node {
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub version: Option<ObjectVersion>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub spec: Option<NodeSpec>,
    pub description: Option<NodeDescription>,
    pub status: Option<NodeStatus>,
    pub manager_status: Option<ManagerStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeSpec {
    pub name: Option<String>,
    pub labels: Option<HashMap<String, String>>,
    pub role: Option<String>,
    pub availability: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeDescription {
    pub hostname: Option<String>,
    pub platform: Option<Platform>,
    pub resources: Option<Resources>,
    pub engine: Option<EngineDescription>,
    pub tls_info: Option<TLSInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Platform {
    pub architecture: Option<String>,
    #[serde(rename = "OS")]
    pub os: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Resources {
    pub nano_cpus: Option<i64>,
    pub memory_bytes: Option<i64>,
    pub generic_resources: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EngineDescription {
    pub engine_version: Option<String>,
    pub labels: Option<HashMap<String, String>>,
    pub plugins: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TLSInfo {
    pub trust_root: Option<String>,
    pub cert_issuer_subject: Option<Vec<u8>>,
    pub cert_issuer_public_key: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NodeStatus {
    pub state: Option<String>,
    pub message: Option<String>,
    pub addr: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ManagerStatus {
    pub leader: Option<bool>,
    pub reachability: Option<String>,
    pub addr: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Service {
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub version: Option<ObjectVersion>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub spec: Option<ServiceSpec>,
    pub endpoint: Option<serde_json::Value>,
    pub update_status: Option<serde_json::Value>,
    pub service_status: Option<serde_json::Value>,
    pub job_status: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServiceSpec {
    pub name: Option<String>,
    pub labels: Option<HashMap<String, String>>,
    pub task_template: Option<TaskSpec>,
    pub mode: Option<serde_json::Value>,
    pub update_config: Option<serde_json::Value>,
    pub rollback_config: Option<serde_json::Value>,
    pub networks: Option<Vec<serde_json::Value>>,
    pub endpoint_spec: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TaskSpec {
    pub container_spec: Option<serde_json::Value>,
    pub network_attachment_spec: Option<serde_json::Value>,
    pub resources: Option<serde_json::Value>,
    pub restart_policy: Option<serde_json::Value>,
    pub placement: Option<serde_json::Value>,
    pub force_update: Option<u64>,
    pub runtime: Option<String>,
    pub networks: Option<Vec<serde_json::Value>>,
    pub log_driver: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Task {
    #[serde(rename = "ID")]
    pub id: Option<String>,
    pub version: Option<ObjectVersion>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub labels: Option<HashMap<String, String>>,
    pub spec: Option<TaskSpec>,
    pub service_id: Option<String>,
    pub slot: Option<i32>,
    pub node_id: Option<String>,
    pub assigned_generic_resources: Option<Vec<serde_json::Value>>,
    pub status: Option<serde_json::Value>,
    pub desired_state: Option<String>,
}
