use std::{cell::Cell, collections::HashMap};

use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use super::{
    environment::{EnvironmentValue, EnvironmentVariable},
    metadata::Metadata,
    volume::{Volume, VolumeMount},
};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    api_version: String,
    kind: String,
    metadata: Metadata,
    template: DeploymentTemplate,
}

#[derive(Debug, Constructor, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentTemplate {
    metadata: DeploymentTemplateMetadata,
    spec: DeploymentTemplateSpec,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentTemplateMetadata {
    namespace: String,
    labels: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentTemplateSpec {
    containers: Vec<DeploymentTemplateContainer>,
    volumes: Vec<Volume>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentTemplateContainer {
    image: String,
    image_pull_policy: String,
    name: String,
    command: Vec<String>,
    ports: Vec<ContainerPort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resources: Option<Value>,
    env: Vec<EnvironmentVariable>,
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

/* --- TRAITS --- */
pub trait DeploymentTemplateMetadataBuilder {
    fn with_label<S: Into<String>>(self, key: S, value: S) -> Self;
    fn build(self) -> DeploymentTemplateMetadata;
}

pub trait DeploymentTemplateContainerBuilder {
    fn with_port(self, port: ContainerPort) -> Self;
    fn with_resources(self, resources: Value) -> Self;
    fn with_env(self, env: EnvironmentVariable) -> Self;
    fn with_volume_mount(self, volume_mount: VolumeMount) -> Self;
    fn build(self) -> DeploymentTemplateContainer;
}

pub trait DeploymentTemplateSpecBuilder {
    fn with_container(self, container: DeploymentTemplateContainer) -> Self;
    fn with_volume(self, volume: Volume) -> Self;
    fn build(self) -> DeploymentTemplateSpec;
}

/* --- IMPLS --- */
impl Deployment {
    pub fn new(metadata: Metadata, template: DeploymentTemplate) -> Self {
        Deployment {
            api_version: "apps/v1".into(),
            kind: "Deployment".into(),
            metadata,
            template,
        }
    }
}

impl DeploymentTemplateMetadata {
    pub fn builder<S: Into<String>>(namepace: S) -> Cell<Self> {
        Cell::new(DeploymentTemplateMetadata {
            namespace: namepace.into(),
            labels: HashMap::default(),
        })
    }
}


impl DeploymentTemplateMetadataBuilder for Cell<DeploymentTemplateMetadata> {
    fn with_label<S: Into<String>>(self, key: S, value: S) -> Self {
        let mut metadata = self.into_inner();
        metadata.labels.insert(key.into(), value.into());
        Cell::new(metadata)
    }

    fn build(self) -> DeploymentTemplateMetadata {
        self.into_inner()
    }
}

impl DeploymentTemplateSpec {
    pub fn builder() -> Cell<Self> {
        Cell::new(DeploymentTemplateSpec {
            containers: Vec::default(),
            volumes: Vec::default(),  
        })
    }
}

impl DeploymentTemplateSpecBuilder for Cell<DeploymentTemplateSpec> {
    fn with_container(self, container: DeploymentTemplateContainer) -> Self {
        let mut spec = self.into_inner();
        spec.containers.push(container);
        Cell::new(spec)
    }

    fn with_volume(self, volume: Volume) -> Self {
        let mut spec = self.into_inner();
        spec.volumes.push(volume);
        Cell::new(spec)
    }

    fn build(self) -> DeploymentTemplateSpec {
        self.into_inner()
    }
}


impl DeploymentTemplateContainer {
    pub fn builder<S: Into<String>>(image: S, name: S, command: Vec<S>) -> Cell<Self> {
        Cell::new(DeploymentTemplateContainer {
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

impl DeploymentTemplateContainerBuilder for Cell<DeploymentTemplateContainer> {
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

    fn build(self) -> DeploymentTemplateContainer {
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
#[cfg(test)]
mod tests {
    use crate::k8s::ast::{environment::{EnvironmentValue, EnvironmentVariable}, metadata::{Metadata, MetadataBuilder}};

    use super::*;

    #[test]
    fn usage() -> anyhow::Result<()> {
        let deployment = Deployment::new(
            Metadata::builder("example-deployment", "example")
                .with_label("example-label", "example-label-value")
                .build(),
            DeploymentTemplate::new(
                DeploymentTemplateMetadata::builder("example").build(),
                DeploymentTemplateSpec::builder()
                    .with_container(DeploymentTemplateContainer::builder("example/image", "example-app", vec!["/usr/bin/app"])
                        .with_env(EnvironmentVariable::new("EXAMPLE_VAR".into(), EnvironmentValue::Static("VALUE".into())))
                        .with_port(ContainerPort::tcp(8080))
                        .with_volume_mount(VolumeMount::new("logs".into(), "/var/logs".into()))
                        .build())
                    .with_volume(Volume::empty_dir("logs".into()))
                    .build(),
            ),
        );

        let yaml = serde_yaml::to_string(&deployment)?;
        println!("{yaml}");

        Ok(())
    }
}
