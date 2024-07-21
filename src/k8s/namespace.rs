use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Namespace {
    api_version: String,
    kind: String,
    metadata: Value,
}

impl Namespace {
    pub fn new(name: String) -> Self {
        Namespace {
            api_version: "v1".into(),
            kind: "Namespace".into(),
            metadata: Value::Mapping(vec![("name".into(), name.into())].into_iter().collect()),
        }
    }
}
