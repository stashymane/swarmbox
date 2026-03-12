use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Network {
    pub name: Option<String>,
    #[serde(rename = "Id")]
    pub id: Option<String>,
    pub created: Option<String>,
    pub scope: Option<String>,
    pub driver: Option<String>,
    pub enable_i_pv6: Option<bool>,
    #[serde(rename = "IPAM")]
    pub ipam: Option<Ipam>,
    pub internal: Option<bool>,
    pub attachable: Option<bool>,
    pub ingress: Option<bool>,
    #[serde(rename = "Containers")]
    pub containers: Option<HashMap<String, NetworkContainer>>,
    pub options: Option<HashMap<String, String>>,
    pub labels: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ipam {
    pub driver: Option<String>,
    pub config: Option<Vec<IpamConfig>>,
    pub options: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IpamConfig {
    pub subnet: Option<String>,
    #[serde(rename = "IPRange")]
    pub ip_range: Option<String>,
    pub gateway: Option<String>,
    pub aux_address: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NetworkContainer {
    pub name: Option<String>,
    pub endpoint_id: Option<String>,
    pub mac_address: Option<String>,
    #[serde(rename = "IPv4Address")]
    pub ipv4_address: Option<String>,
    #[serde(rename = "IPv6Address")]
    pub ipv6_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EndpointSettings {
    #[serde(rename = "IPAMConfig")]
    pub ipam_config: Option<EndpointIpamConfig>,
    pub links: Option<Vec<String>>,
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "NetworkID")]
    pub network_id: Option<String>,
    #[serde(rename = "EndpointID")]
    pub endpoint_id: Option<String>,
    pub gateway: Option<String>,
    #[serde(rename = "IPAddress")]
    pub ip_address: Option<String>,
    pub ip_prefix_len: Option<i32>,
    #[serde(rename = "IPv6Gateway")]
    pub ipv6_gateway: Option<String>,
    #[serde(rename = "GlobalIPv6Address")]
    pub global_ipv6_address: Option<String>,
    pub global_ipv6_prefix_len: Option<i64>,
    pub mac_address: Option<String>,
    pub driver_opts: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EndpointIpamConfig {
    #[serde(rename = "IPv4Address")]
    pub ipv4_address: Option<String>,
    #[serde(rename = "IPv6Address")]
    pub ipv6_address: Option<String>,
    pub link_local_ips: Option<Vec<String>>,
}
