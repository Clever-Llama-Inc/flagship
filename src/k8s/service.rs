use std::cell::Cell;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

use super::{metadata::Metadata, selector::Selector};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    api_version: String,
    kind: String,
    metadata: Metadata,
    spec: ServiceSpec,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceSpec {
    service_type: ServiceType,
    selector: Selector,
    ports: Vec<ServicePort>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum ServiceType {
    LoadBalancer,
    ClusterIP,
}

#[derive(Debug, Constructor, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ServicePort {
    port: u16,
    target_port: u16,
}

impl Service {
    pub fn new(metadata: Metadata, spec: ServiceSpec) -> Self {
        Service {
            api_version: "apps/v1".into(),
            kind: "Service".into(),
            metadata,
            spec,
        }
    }
}

impl ServiceSpec {
    pub fn builder(service_type: ServiceType, selector: Selector) -> Cell<ServiceSpec> {
        Cell::new(ServiceSpec {
            service_type,
            selector,
            ports: Vec::default(),
        })
    }
}

pub trait ServiceSpecBuilder {
    fn with_port(self, port: u16, target_port: u16) -> Self;
    fn build(self) -> ServiceSpec;
}

impl ServiceSpecBuilder for Cell<ServiceSpec> {
    fn with_port(self, port: u16, target_port: u16) -> Self {
        let mut spec = self.into_inner();
        spec.ports.push(ServicePort::new(port, target_port));
        Cell::new(spec)
    }
    
    fn build(self) -> ServiceSpec {
        self.into_inner()
    }
}