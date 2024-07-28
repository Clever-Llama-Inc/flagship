use std::cell::Cell;

use crate::prelude::*;
use derive_more::Constructor;
use serde_yaml::Value;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StackError {
    #[error("SerdeYamlError: {0}")]
    SerdeYamlError(#[from] serde_yaml::Error),
}

pub type StackResult<T> = Result<T, StackError>;

#[derive(Debug, Constructor)]
pub struct Stack {
    namespace: String,
    environment: Environment,
    resources: Vec<Resource>,
}

#[derive(Debug)]
pub enum Environment {
    Production,
    Ephemeral(String),
}

#[derive(Debug)]
pub enum Resource {
    PosgreSQL(PostgreSQL),
    RabbitMQ(RabbitMQ),
    Nginx(Nginx),
    Microservice(Microservice),
}

#[derive(Debug, Constructor)]
pub struct PostgreSQL {
    image: String,
}

#[derive(Debug, Constructor)]
pub struct RabbitMQ {
    image: String,
}

#[derive(Debug, Constructor)]
pub struct Microservice {
    image: String,
    version: String,
    replicas: u16,
    role: String,
    tcp_ports: Vec<MicroservicePort>,
}

#[derive(Debug)]
pub enum MicroservicePort {
    TCP { port: u16, name: Option<String> },
    UDP { port: u16, name: Option<String> }
}

#[derive(Debug, Constructor)]
pub struct Nginx {
    image: String,
    replicas: u16,
}

impl Stack {
    pub fn builder<S: Into<String>>(name: S, environment: Environment) -> Cell<Stack> {
        Cell::new(Stack {
            namespace: name.into(),
            environment,
            resources: Vec::default(),
        })
    }

    fn namespace(&self) -> Vec<String> {
        match &self.environment {
            Environment::Production => vec![self.namespace.clone()],
            Environment::Ephemeral(name) => vec![self.namespace.clone(), name.clone()],
        }
    }

    fn postgresql(&self, pg: &PostgreSQL) -> StackResult<Vec<Value>> {
        let ns = self.namespace().join("-");
        let app_name = [ns.clone(), "db".to_string()].join("-");
        let service_name = [app_name.clone(), "svc".to_string()].join("-");
        let volume_name = [app_name.clone(), "vol".to_string()].join("-");

        let metadata = Metadata::builder(app_name.clone(), ns.clone())
            .with_label("app", &app_name)
            .build();

        let stateful_set = StatefulSet::new(
            metadata.clone(),
            StatefulSetSpec::builder(
                service_name.clone(),
                Selector::builder()
                    .with_match_label("app", &app_name)
                    .build(),
                StatefulSetSpecTemplate::new(
                    metadata.clone(),
                    StatefulSetSpecTemplateSpec::builder()
                        .with_container(
                            Container::builder(pg.image.clone(), app_name.clone(), Vec::default())
                                .with_port(ContainerPort::tcp(5432))
                                .with_env(EnvironmentVariable::new(
                                    "POSTGRES_PASSWORD".into(),
                                    EnvironmentValue::Static("postgres".into()),
                                ))
                                .with_volume_mount(VolumeMount::new(
                                    volume_name.clone(),
                                    "/var/lib/postgresql/data".to_string(),
                                ))
                                .build(),
                        )
                        .build(),
                ),
            )
            .with_volume_claim_template(VolumeClaimTemplate::new(
                Metadata::builder(volume_name.clone(), ns.clone()).build(),
                VolumeClaimTemplateSpec::builder(VolumeClaimTemplateSpec::storage_resources(
                    "5Gi".to_string(),
                ))
                .with_access_mode(VolumeClaimAccessMode::ReadWriteOnce)
                .build(),
            ))
            .build(),
        );

        let service = Service::new(
            Metadata::builder(service_name.clone(), ns.clone())
                .with_label("app", &app_name.clone())
                .build(),
            ServiceSpec::builder(ServiceType::ClusterIP)
                .with_selector("app", &app_name)
                .with_port(5432, 5432, Some("db"))
                .build(),
        );

        Ok(vec![
            serde_yaml::to_value(&stateful_set)?,
            serde_yaml::to_value(&service)?,
        ])
    }

    fn rabbitmq(&self, rmq: &RabbitMQ) -> StackResult<Vec<Value>> {
        let ns: String = self.namespace().join("-");
        let app_name = [ns.clone(), "mq".to_string()].join("-");
        let service_name = [app_name.clone(), "svc".to_string()].join("-");
        let volume_name = [app_name.clone(), "vol".to_string()].join("-");

        let metadata = Metadata::builder(app_name.clone(), ns.clone())
            .with_label("app", &app_name)
            .build();

        let stateful_set = StatefulSet::new(
            metadata.clone(),
            StatefulSetSpec::builder(
                service_name.clone(),
                Selector::builder()
                    .with_match_label("app", &app_name)
                    .build(),
                StatefulSetSpecTemplate::new(
                    metadata.clone(),
                    StatefulSetSpecTemplateSpec::builder()
                        .with_container(
                            Container::builder(rmq.image.clone(), app_name.clone(), Vec::default())
                                .with_port(ContainerPort::tcp(5672))
                                .with_port(ContainerPort::tcp(15672))
                                .with_volume_mount(VolumeMount::new(
                                    volume_name.clone(),
                                    "/var/lib/postgresql/data".to_string(),
                                ))
                                .build(),
                        )
                        .build(),
                ),
            )
            .with_volume_claim_template(VolumeClaimTemplate::new(
                Metadata::builder(volume_name.clone(), ns.clone()).build(),
                VolumeClaimTemplateSpec::builder(VolumeClaimTemplateSpec::storage_resources(
                    "5Gi".to_string(),
                ))
                .with_access_mode(VolumeClaimAccessMode::ReadWriteOnce)
                .build(),
            ))
            .build(),
        );

        let service = Service::new(
            Metadata::builder(service_name.clone(), ns.clone())
                .with_label("app", &app_name.clone())
                .build(),
            ServiceSpec::builder(ServiceType::ClusterIP)
                .with_selector("app", &app_name)
                .with_port(5672, 5672, Some("amqp"))
                .with_port(15672, 15672, Some("web"))
                .build(),
        );

        Ok(vec![
            serde_yaml::to_value(&stateful_set)?,
            serde_yaml::to_value(&service)?,
        ])
    }

    fn nginx(&self, nginx: &Nginx) -> StackResult<Vec<Value>> {
        let ns: String = self.namespace().join("-");
        let app_name = [ns.clone(), "web".to_string()].join("-");
        let service_name = [app_name.clone(), "svc".to_string()].join("-");

        let metadata = Metadata::builder(app_name.clone(), ns.clone())
            .with_label("app", &app_name)
            .build();
        let deployment = Deployment::new(
            metadata.clone(),
            DeploymentSpec::new(
                nginx.replicas,
                Selector::builder()
                    .with_match_label("app", &app_name)
                    .build(),
                DeploymentTemplate::new(
                    DeploymentTemplateMetadata::builder(ns.clone())
                        .with_label("app", &app_name)
                        .build(),
                    DeploymentTemplateSpec::builder()
                        .with_container(
                            Container::builder(&nginx.image, &app_name, Vec::default())
                                .with_port(ContainerPort::tcp(80))
                                .build(),
                        )
                        .build(),
                ),
            ),
        );

        let service = Service::new(
            Metadata::builder(service_name.clone(), ns.clone())
                .with_label("app", &app_name)
                .build(),
            ServiceSpec::builder(ServiceType::LoadBalancer)
                .with_selector("app", &app_name)
                .with_port(80, 80, Some("web"))
                .build(),
        );

        Ok(vec![
            serde_yaml::to_value(&deployment)?,
            serde_yaml::to_value(&service)?,
        ])
    }

    fn microservice(&self, microservice: &Microservice) -> StackResult<Vec<Value>> {
        let ns: String = self.namespace().join("-");
        let app_name = [ns.clone(), microservice.role.clone()].join("-");
        let service_name = [app_name.clone(), "svc".to_string()].join("-");

        let metadata = Metadata::builder(app_name.clone(), ns.clone())
            .with_label("app", &app_name)
            .with_label("role", &microservice.role)
            .with_label("version", &microservice.version)
            .build();
        let deployment = Deployment::new(
            metadata.clone(),
            DeploymentSpec::new(
                microservice.replicas,
                Selector::builder()
                    .with_match_label("app", &app_name)
                    .build(),
                DeploymentTemplate::new(
                    DeploymentTemplateMetadata::builder(ns.clone())
                        .with_label("app", &app_name)
                        .build(),
                    DeploymentTemplateSpec::builder()
                        .with_container({
                            let mut c =
                                Container::builder(&microservice.image, &app_name, Vec::default());
                            for port in &microservice.tcp_ports {
                                let port = match port {
                                    MicroservicePort::TCP { port, .. } => ContainerPort::tcp(*port),
                                    MicroservicePort::UDP { port, .. } => ContainerPort::udp(*port),
                                };
        
                                c = c.with_port(port);
                            }
                            c.build()
                        })
                        .build(),
                ),
            ),
        );

        let mut values = Vec::default();
        values.push(serde_yaml::to_value(&deployment)?);

        if !microservice.tcp_ports.is_empty() {
            let service = Service::new(
                Metadata::builder(service_name.clone(), ns.clone())
                    .with_label("app", &app_name)
                    .build(),
                {
                    let mut spec = ServiceSpec::builder(ServiceType::LoadBalancer)
                        .with_selector("app", &app_name);
                    for port in &microservice.tcp_ports {
                        let (port, name) = match port {
                            MicroservicePort::TCP { port, name } => (*port, name.clone()),
                            MicroservicePort::UDP { port, name } => (*port, name.clone()),
                        };
                        spec = spec.with_port(port, port, name);
                    }
                    spec.build()
                },
            );
            values.push(serde_yaml::to_value(&service)?)
        }

        Ok(values)
    }

    pub fn as_k8s(&self) -> StackResult<Vec<Value>> {
        let mut values = Vec::default();
        let ns = serde_yaml::to_value(Namespace::new(self.namespace().join("-")))?;
        values.push(ns);

        self.resources.iter().try_fold(values, |mut vs, r| {
            let mut v = match r {
                Resource::PosgreSQL(pg) => self.postgresql(pg)?,
                Resource::RabbitMQ(rmq) => self.rabbitmq(rmq)?,
                Resource::Nginx(nginx) => self.nginx(nginx)?,
                Resource::Microservice(ms) => self.microservice(ms)?,
            };
            vs.append(&mut v);
            Ok(vs)
        })
    }
}

pub trait StackBuilder {
    fn with_resource(self, resource: Resource) -> Self;
    fn build(self) -> Stack;
}

impl StackBuilder for Cell<Stack> {
    fn with_resource(self, resource: Resource) -> Self {
        let mut stack = self.into_inner();
        stack.resources.push(resource);
        Cell::new(stack)
    }

    fn build(self) -> Stack {
        self.into_inner()
    }
}
