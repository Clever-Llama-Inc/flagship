use std::{cell::Cell, collections::HashMap};

use serde::{Deserialize, Serialize};
use serde_yaml::Value;

use super::metadata::Metadata;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Secret {
    api_version: String,
    kind: String,
    metadata: Metadata,
    #[serde(rename = "type")]
    secret_type: SecretType,
    data: HashMap<String, Value>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SecretType {
    Opaque
}

pub trait SecretBuilder {
    fn with_data<S: Into<String>>(self, key: S, value: Value) -> Self;
    fn build(self) -> Secret;
}

impl Secret { 
    pub fn builder(secret_type: SecretType, metadata: Metadata) -> Cell<Secret> {
        Cell::new(Secret {
            api_version: "v1".into(),
            kind: "Secret".into(),
            metadata,
            secret_type,
            data: HashMap::default(),
        })
    }
}

impl SecretBuilder for Cell<Secret> {
    fn with_data<S: Into<String>>(self, key: S, value: Value) -> Self {
        let mut secret = self.into_inner();
        secret.data.insert(key.into(), value);
        Cell::new(secret)
    }

    fn build(self) -> Secret {
        self.into_inner()
    }
}