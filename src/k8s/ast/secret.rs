use std::collections::HashMap;

use serde::{Deserialize, Serialize};

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

pub enum SecretType {
    Opaque
}