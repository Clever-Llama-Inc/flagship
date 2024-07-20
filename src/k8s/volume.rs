use std::cell::Cell;

use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};

use super::metadata::Metadata;

#[derive(Debug, Constructor, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeMount {
    name: String,
    mount_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    name: String,
    empty_dir: Option<Value>,
    config_map: Option<Value>,
}

#[derive(Debug, Constructor, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeClaimTemplate {
    metadata: Metadata,
    spec: VolumeClaimTemplateSpec
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VolumeClaimTemplateSpec {
    access_modes: Vec<VolumeClaimAccessMode>,
    resources: Value,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum VolumeClaimAccessMode {
    ReadWriteOnce,
}

impl Volume {
    pub fn empty_dir(name: String) -> Self {
        Volume {
            name,
            empty_dir: Some(Mapping::default().into()),
            config_map: None,
        }
    }

    pub fn config_map(name: String, config_map_name: String) -> Self {
        Volume {
            name,
            empty_dir: None,
            config_map: Some(Value::Mapping(
                vec![(
                    "configMap".into(),
                    Value::Mapping(
                        vec![("name".into(), config_map_name.into())]
                            .into_iter()
                            .collect(),
                    ),
                )]
                .into_iter()
                .collect(),
            )),
        }
    }
}

impl VolumeClaimTemplateSpec {
    pub fn builder(resources: Value) -> Cell<Self> {
        Cell::new(VolumeClaimTemplateSpec {
            access_modes: Vec::default(),
            resources,
        })
    }

    pub fn storage_resources(amount: String) -> Value {
        Value::Mapping(
            vec![("requests".into(), Value::Mapping(vec![
                ("storage".into(), amount.into())
            ].into_iter().collect()))].into_iter().collect()
        )
    }
}

pub trait VolumeClaimTemplateSpecBuilder {
    fn with_access_mode(self, access_mode: VolumeClaimAccessMode) -> Self;
    fn build(self) -> VolumeClaimTemplateSpec;
}

impl VolumeClaimTemplateSpecBuilder for Cell<VolumeClaimTemplateSpec> {
    fn with_access_mode(self, access_mode: VolumeClaimAccessMode) -> Self {
        let mut spec = self.into_inner();
        spec.access_modes.push(access_mode);
        Cell::new(spec)
    }
    
    fn build(self) -> VolumeClaimTemplateSpec {
        self.into_inner()
    }

    
}