use std::cell::Cell;

use super::{
    container::Container, metadata::Metadata, selector::Selector, volume::VolumeClaimTemplate,
};
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSet {
    api_version: String,
    kind: String,
    metadata: Metadata,
    spec: StatefulSetSpec,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetSpec {
    service_name: String,
    selector: Selector,
    template: StatefulSetSpecTemplate,
    volume_claim_templates: Vec<VolumeClaimTemplate>,
}

#[derive(Debug, Constructor, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetSpecTemplate {
    metadata: Metadata,
    spec: StatefulSetSpecTemplateSpec,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatefulSetSpecTemplateSpec {
    containers: Vec<Container>,
}

impl StatefulSet {
    pub fn new(metadata: Metadata, spec: StatefulSetSpec) -> Self {
        StatefulSet {
            api_version: "apps/v1".into(),
            kind: "StatefulSet".into(),
            metadata,
            spec,
        }
    }
}

impl StatefulSetSpec {
    pub fn builder(
        service_name: String,
        selector: Selector,
        template: StatefulSetSpecTemplate,
    ) -> Cell<Self> {
        Cell::new(StatefulSetSpec {
            service_name,
            selector,
            template,
            volume_claim_templates: Vec::new(),
        })
    }
}

pub trait StatefulSetSpecBuilder {
    fn with_volume_claim_template(self, volume_claim_template: VolumeClaimTemplate) -> Self;
    fn build(self) -> StatefulSetSpec;
}

impl StatefulSetSpecBuilder for Cell<StatefulSetSpec> {
    fn with_volume_claim_template(self, volume_claim_template: VolumeClaimTemplate) -> Self {
        let mut template = self.into_inner();
        template.volume_claim_templates.push(volume_claim_template);
        Cell::new(template)
    }

    fn build(self) -> StatefulSetSpec {
        self.into_inner()
    }
}

impl StatefulSetSpecTemplateSpec {
    pub fn builder() -> Cell<StatefulSetSpecTemplateSpec> {
        Cell::new(StatefulSetSpecTemplateSpec {
            containers: Vec::default(),
        })
    }
}

pub trait StatefulSetSpecTemplateSpecBuilder {
    fn with_container(self, container: Container) -> Self;
    fn build(self) -> StatefulSetSpecTemplateSpec;
}

impl StatefulSetSpecTemplateSpecBuilder for Cell<StatefulSetSpecTemplateSpec> {
    fn with_container(self, container: Container) -> Self {
        let mut spec = self.into_inner();
        spec.containers.push(container);
        Cell::new(spec)
    }

    fn build(self) -> StatefulSetSpecTemplateSpec {
        self.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn usage() -> anyhow::Result<()> {
        let stateful_set = StatefulSet::new(
            Metadata::builder("database", "example")
                .with_label("app", "example-db")
                .build(),
            StatefulSetSpec::builder(
                "example-db".into(),
                Selector::builder()
                    .with_match_label("app", "example-db")
                    .build(),
                StatefulSetSpecTemplate::new(
                    Metadata::builder("example-db", "example")
                        .with_label("app", "example-db")
                        .build(),
                    StatefulSetSpecTemplateSpec::builder()
                        .with_container(
                            Container::builder("pgsql", "example-db", vec!["/usr/bin/postgres"])
                                .with_env(("PG_USER", "example").into())
                                .with_port(ContainerPort::tcp(5432))
                                .with_volume_mount(VolumeMount::new(
                                    "example-db-vol".into(),
                                    "/var/lib/data".into(),
                                ))
                                .build(),
                        )
                        .build(),
                ),
            )
            .with_volume_claim_template(VolumeClaimTemplate::new(
                Metadata::builder("example-db-vol", "example").build(),
                VolumeClaimTemplateSpec::builder(VolumeClaimTemplateSpec::storage_resources(
                    "5Gi".into(),
                ))
                .with_access_mode(VolumeClaimAccessMode::ReadWriteOnce)
                .build(),
            ))
            .build(),
        );

        let yaml = serde_yaml::to_string(&stateful_set)?;

        println!("{yaml}");

        Ok(())
    }
}
