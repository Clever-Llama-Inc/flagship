use std::{cell::Cell, collections::HashMap};

use derive_more::Constructor;
use serde::{Deserialize, Serialize};

use super::{container::Container, metadata::Metadata, selector::Selector, volume::Volume};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    api_version: String,
    kind: String,
    metadata: Metadata,
    spec: DeploymentSpec,
}

#[derive(Debug, Constructor, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentSpec {
    replicas: u16,
    selector: Selector,
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
    containers: Vec<Container>,
    volumes: Vec<Volume>,
}

/* --- TRAITS --- */
pub trait DeploymentTemplateMetadataBuilder {
    fn with_label<S: Into<String>>(self, key: S, value: S) -> Self;
    fn build(self) -> DeploymentTemplateMetadata;
}

pub trait DeploymentTemplateSpecBuilder {
    fn with_container(self, container: Container) -> Self;
    fn with_volume(self, volume: Volume) -> Self;
    fn build(self) -> DeploymentTemplateSpec;
}

/* --- IMPLS --- */
impl Deployment {
    pub fn new(metadata: Metadata, spec: DeploymentSpec) -> Self {
        Deployment {
            api_version: "apps/v1".into(),
            kind: "Deployment".into(),
            metadata,
            spec,
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
    fn with_container(self, container: Container) -> Self {
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

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::*;

    #[test]
    fn usage() -> anyhow::Result<()> {
        let deployment = Deployment::new(
            Metadata::builder("example-deployment", "example")
                .with_label("example-label", "example-label-value")
                .build(),
            DeploymentSpec::new(
                3,
                Selector::builder()
                    .with_match_label("app", "example-api-svc")
                    .build(),
                DeploymentTemplate::new(
                    DeploymentTemplateMetadata::builder("example")
                        .with_label("app", "example-api")
                        .build(),
                    DeploymentTemplateSpec::builder()
                        .with_container(
                            Container::builder(
                                "example/image",
                                "example-app",
                                vec!["/usr/bin/app"],
                            )
                            .with_env(EnvironmentVariable::new(
                                "EXAMPLE_VAR".into(),
                                EnvironmentValue::Static("example value".into()),
                            ))
                            .with_port(ContainerPort::tcp(8080))
                            .with_volume_mount(VolumeMount::new("logs".into(), "/var/logs".into()))
                            .build(),
                        )
                        .with_volume(Volume::empty_dir("logs".into()))
                        .build(),
                ),
            ),
        );

        let yaml = serde_yaml::to_string(&deployment)?;
        println!("{yaml}");

        Ok(())
    }
}
