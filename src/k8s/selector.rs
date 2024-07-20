use std::{cell::Cell, collections::HashMap};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Selector {
    match_labels: HashMap<String, String>
}

pub trait SelectorBuilder {
    fn with_match_label<S: Into<String>>(self, key: S, value: S) -> Self;
    fn build(self) -> Selector;
}

impl Selector {
    pub fn builder() -> Cell<Self> {
        Cell::new(Selector {
            match_labels: HashMap::default()
        })
    }
}

impl SelectorBuilder for Cell<Selector> {
    fn with_match_label<S: Into<String>>(self, key: S, value: S) -> Self {
        let mut selector = self.into_inner();
        selector.match_labels.insert(key.into(), value.into());
        Cell::new(selector)
    }

    fn build(self) -> Selector {
        self.into_inner()
    }
}