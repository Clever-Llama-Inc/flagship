use std::cell::Cell;

use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use super::{environment::EnvironmentVariable, volume::VolumeMount};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    image: String,
    image_pull_policy: String,
    name: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    command: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    ports: Vec<ContainerPort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<Value>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    env: Vec<EnvironmentVariable>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    volume_mounts: Vec<VolumeMount>,
}

#[derive(Debug, Constructor, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerPort {
    container_port: u16,
    protocol: ContainerPortProtocol,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ContainerPortProtocol {
    TCP,
    UDP,
}

impl Container {
    pub fn builder<S: Into<String>>(image: S, name: S, command: Vec<S>) -> Cell<Self> {
        Cell::new(Container {
            image: image.into(),
            image_pull_policy: "IfNotPresent".into(),
            name: name.into(),
            command: command.into_iter().map(S::into).collect(),
            ports: Vec::default(),
            resources: None,
            env: Vec::default(),
            volume_mounts: Vec::default(),
        })
    }
}

pub trait ContainerBuilder {
    fn with_port(self, port: ContainerPort) -> Self;
    fn with_resources(self, resources: Value) -> Self;
    fn with_env(self, env: EnvironmentVariable) -> Self;
    fn with_volume_mount(self, volume_mount: VolumeMount) -> Self;
    fn build(self) -> Container;
}

impl ContainerBuilder for Cell<Container> {
    fn with_port(self, port: ContainerPort) -> Self {
        let mut container = self.into_inner();
        container.ports.push(port);
        Cell::new(container)
    }

    fn with_resources(self, resources: Value) -> Self {
        let mut container = self.into_inner();
        container.resources = Some(resources);
        Cell::new(container)
    }

    fn with_env(self, env: EnvironmentVariable) -> Self {
        let mut container = self.into_inner();
        container.env.push(env);
        Cell::new(container)
    }

    fn with_volume_mount(self, volume_mount: VolumeMount) -> Self {
        let mut container = self.into_inner();
        container.volume_mounts.push(volume_mount);
        Cell::new(container)
    }

    fn build(self) -> Container {
        self.into_inner()
    }
}

impl ContainerPort {
    pub fn tcp(port: u16) -> Self {
        ContainerPort {
            container_port: port,
            protocol: ContainerPortProtocol::TCP,
        }
    }

    pub fn udp(port: u16) -> Self {
        ContainerPort {
            container_port: port,
            protocol: ContainerPortProtocol::UDP,
        }
    }
}
