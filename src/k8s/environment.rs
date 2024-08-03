use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EnvironmentVariable {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value_from: Option<ValueFrom>,
}

impl<Name: Into<String>, Value: Into<String>> From<(Name, Value)> for EnvironmentVariable {
    fn from((name, value): (Name, Value)) -> Self {
        EnvironmentVariable::value(name, value)
    }
}

impl EnvironmentVariable {
    pub fn value<Name: Into<String>, Value: Into<String>>(name: Name, value: Value) -> Self {
        Self {
            name: name.into(),
            value: Some(value.into()),
            value_from: None,   
        }
    }

    pub fn value_from<Name: Into<String>>(name: Name, value_from: ValueFrom) -> Self {
        Self {
            name: name.into(),
            value: None,
            value_from: Some(value_from),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ValueFrom {
    FieldRef { field_path: String },
    SecretKeyRef { key: String, name: String, optional: bool },
    ResourceFieldRef { resource: String, container_name: String, divisor: String }
}

mod tests {
    #[test]
    fn test_serialize() {
        let env = vec![
            super::EnvironmentVariable::value("TEST", "test"),
            super::EnvironmentVariable::value_from("FIELD", super::ValueFrom::FieldRef {field_path: "metadata.name".into()}),
            super::EnvironmentVariable::value_from("SECRET", super::ValueFrom::SecretKeyRef{
                key: "key".into(),
                name: "secret".into(),
                optional: false
            }),
        ];

        let yaml = serde_yaml::to_string(&env).unwrap();
        println!("{yaml}");
    }
}