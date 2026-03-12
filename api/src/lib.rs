pub mod models;

#[cfg(test)]
mod tests {
    use super::models::container::{ContainerConfig, HostConfig};
    use std::collections::HashMap;

    #[test]
    fn test_container_config_serialization() {
        let mut exposed_ports = HashMap::new();
        exposed_ports.insert("80/tcp".to_string(), HashMap::new());

        let config = ContainerConfig {
            image: Some("ubuntu:latest".to_string()),
            exposed_ports: Some(exposed_ports),
            env: Some(vec!["FOO=BAR".to_string()]),
            ..Default::default()
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"Image\":\"ubuntu:latest\""));
        assert!(json.contains("\"ExposedPorts\":{\"80/tcp\":{}}"));
    }

    #[test]
    fn test_host_config_serialization() {
        let config = HostConfig {
            memory: Some(1024 * 1024 * 512),
            network_mode: Some("bridge".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&config).unwrap();
        assert!(json.contains("\"Memory\":536870912"));
        assert!(json.contains("\"NetworkMode\":\"bridge\""));
    }

    #[test]
    fn test_image_inspect_serialization() {
        use super::models::image::ImageInspect;
        use super::models::image::RootFs;

        let inspect = ImageInspect {
            id: Some("sha256:1234567890".to_string()),
            rootfs: Some(RootFs {
                rootfs_type: "layers".to_string(),
                layers: Some(vec!["sha256:layer1".to_string()]),
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&inspect).unwrap();
        assert!(json.contains("\"Id\":\"sha256:1234567890\""));
        assert!(json.contains("\"RootFS\":{\"Type\":\"layers\""));
    }

    #[test]
    fn test_network_serialization() {
        use super::models::network::{Ipam, IpamConfig, Network};

        let network = Network {
            name: Some("test-net".to_string()),
            ipam: Some(Ipam {
                driver: Some("default".to_string()),
                config: Some(vec![IpamConfig {
                    subnet: Some("172.18.0.0/16".to_string()),
                    ..Default::default()
                }]),
                options: None,
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&network).unwrap();
        assert!(json.contains("\"Name\":\"test-net\""));
        assert!(json.contains("\"IPAM\":{\"Driver\":\"default\""));
    }
}

impl Default for models::image::ImageInspect {
    fn default() -> Self {
        Self {
            id: None,
            repo_tags: None,
            repo_digests: None,
            parent: None,
            comment: None,
            created: None,
            container: None,
            container_config: None,
            docker_version: None,
            author: None,
            config: None,
            architecture: None,
            os: None,
            os_version: None,
            size: None,
            virtual_size: None,
            graph_driver: None,
            rootfs: None,
            metadata: None,
        }
    }
}

impl Default for models::network::Network {
    fn default() -> Self {
        Self {
            name: None,
            id: None,
            created: None,
            scope: None,
            driver: None,
            enable_i_pv6: None,
            ipam: None,
            internal: None,
            attachable: None,
            ingress: None,
            containers: None,
            options: None,
            labels: None,
        }
    }
}

impl Default for models::network::IpamConfig {
    fn default() -> Self {
        Self {
            subnet: None,
            ip_range: None,
            gateway: None,
            aux_address: None,
        }
    }
}

impl Default for models::container::ContainerConfig {
    fn default() -> Self {
        Self {
            hostname: None,
            domainname: None,
            user: None,
            attach_stdin: None,
            attach_stdout: None,
            attach_stderr: None,
            exposed_ports: None,
            tty: None,
            open_stdin: None,
            stdin_once: None,
            env: None,
            cmd: None,
            healthcheck: None,
            args_escaped: None,
            image: None,
            volumes: None,
            working_dir: None,
            entrypoint: None,
            network_disabled: None,
            mac_address: None,
            on_build: None,
            labels: None,
            stop_signal: None,
            stop_timeout: None,
            shell: None,
        }
    }
}

impl Default for models::container::HostConfig {
    fn default() -> Self {
        Self {
            cpu_shares: None,
            memory: None,
            cgroup_parent: None,
            blkio_weight: None,
            blkio_weight_device: None,
            blkio_device_read_bps: None,
            blkio_device_write_bps: None,
            blkio_device_read_iops: None,
            blkio_device_write_iops: None,
            cpu_period: None,
            cpu_quota: None,
            cpu_realtime_period: None,
            cpu_realtime_runtime: None,
            cpuset_cpus: None,
            cpuset_mems: None,
            devices: None,
            device_cgroup_rules: None,
            device_requests: None,
            kernel_memory_tcp: None,
            memory_reservation: None,
            memory_swap: None,
            memory_swappiness: None,
            nano_cpus: None,
            oom_kill_disable: None,
            init: None,
            pids_limit: None,
            ulimits: None,
            cpu_count: None,
            cpu_percent: None,
            io_maximum_iops: None,
            io_maximum_bandwidth: None,
            binds: None,
            container_id_file: None,
            log_config: None,
            network_mode: None,
            port_bindings: None,
            restart_policy: None,
            auto_remove: None,
            volume_driver: None,
            volumes_from: None,
            mounts: None,
            console_size: None,
            annotations: None,
            cap_add: None,
            cap_drop: None,
            dns: None,
            dns_options: None,
            dns_search: None,
            extra_hosts: None,
            group_add: None,
            ipc_mode: None,
            cgroup: None,
            links: None,
            oom_score_adj: None,
            pid_mode: None,
            privileged: None,
            publish_all_ports: None,
            readonly_rootfs: None,
            security_opt: None,
            storage_opt: None,
            tmpfs: None,
            uts_mode: None,
            userns_mode: None,
            shm_size: None,
            sysctls: None,
            runtime: None,
            isolation: None,
            masked_paths: None,
            readonly_paths: None,
        }
    }
}
