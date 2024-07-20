use std::{cell::Cell, collections::HashMap};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    name: String,
    namespace: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    labels: HashMap<String, String>,
}

impl Metadata {
    pub fn builder<S: Into<String>>(name: S, namespace: S) -> Cell<Metadata> {
        Cell::new(Metadata {
            name: name.into(),
            namespace: namespace.into(),
            labels: HashMap::default(),
        })
    }
}

pub trait MetadataBuilder {
    fn with_label<S: Into<String>>(self, key: S, value: S) -> Self;
    fn build(self) -> Metadata;
}

impl MetadataBuilder for Cell<Metadata> {
    fn with_label<S: Into<String>>(self, key: S, value: S) -> Self {
        let mut m = self.into_inner();
        m.labels.insert(key.into(), value.into());
        Cell::new(m)
    }

    fn build(self) -> Metadata {
        self.into_inner()
    }
}
