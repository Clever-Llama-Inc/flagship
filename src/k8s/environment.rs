use serde::{Deserialize, Serialize};
use derive_more::Constructor;

#[derive(Debug, Constructor, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentVariable {
    name: String,
    value: EnvironmentValue
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase", untagged)]
pub enum EnvironmentValue {
    Static(String),
    SecretKeyRef { name: String, key: String }
}