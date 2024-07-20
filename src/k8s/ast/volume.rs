use serde::{Deserialize, Serialize};
use derive_more::Constructor;
use serde_yaml::Value;

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

impl Volume {
    pub fn empty_dir(name: String) -> Self {
        Volume {
            name,
            empty_dir: Some(Value::String("{}".to_string())),
            config_map: None,
        }
    }

    pub fn config_map(name: String, config_map_name: String) -> Self {
        Volume {
            name,
            empty_dir: None,
            config_map: Some(Value::Mapping(vec![
                ("configMap".into(), Value::Mapping(vec![
                    ("name".into(), config_map_name.into())
                ].into_iter().collect()))
            ].into_iter().collect()))
        }
    }
}