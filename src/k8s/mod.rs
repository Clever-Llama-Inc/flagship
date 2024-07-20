pub mod ast;

use derive_more::*;
use serde_yaml::Value as V;

use crate::{
    Cluster, ClusterConfig, Command, Directory, Environment, EnvironmentValue, Image, Name,
    PostgreSQL, RabbitMQ, Resource, StatelessApplication,
};



#[derive(Constructor)]
struct ResourceContext<'c> {
    config: &'c ClusterConfig,
    resource: &'c Resource,
}

impl From<&Cluster> for Vec<V> {
    fn from(value: &Cluster) -> Self {
        value
            .resources
            .iter()
            .map(|r| ResourceContext::new(&value.cluster_config, r))
            .flat_map(|rc| Vec::<V>::from(rc))
            .collect::<Vec::<V>>()
    }
}

fn from_postgres(_pgsql: &PostgreSQL) -> V {
    todo!()
}

fn from_rabbitmq(_rmq: &RabbitMQ) -> V {
    todo!()
}

fn from_stateless_application(app: &StatelessApplication, config: &ClusterConfig) -> Vec<V> {
    vec![
        V::Mapping(
            vec![
                ("apiVersion".into(), "apps/v1".into()),
                ("kind".into(), "Deployment".into()),
                (
                    "metadata".into(),
                    V::Mapping(
                        vec![
                            ("name".into(), (&app.name).into()),
                            ("namespace".into(), (&config.cluster_name).into()),
                            (
                                "labels".into(),
                                V::Mapping(
                                    vec![("app".into(), (&app.name).into())]
                                        .into_iter()
                                        .collect(),
                                ),
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                ),
                (
                    "spec".into(),
                    V::Mapping(
                        vec![
                            (
                                "containers".into(),
                                V::Sequence(vec![V::Mapping(
                                    vec![
                                        ("image".into(), (&app.image).into()),
                                        ("imagePullPolicy".into(), "IfNotPresent".into()),
                                        ("command".into(), (&app.command).into()),
                                        // TODO: PORTS
                                        ("env".into(), (&app.environment).into()),
                                        (
                                            "volumeMounts".into(),
                                            V::Sequence(vec![V::Mapping(
                                                vec![
                                                    ("name".into(), "logs".into()),
                                                    ("mountPath".into(), (&app.logs).into()),
                                                ]
                                                .into_iter()
                                                .collect(),
                                            )]),
                                        ),
                                    ]
                                    .into_iter()
                                    .collect(),
                                )]),
                            ),
                            (
                                "volumes".into(),
                                V::Sequence(
                                    vec![V::Mapping(
                                        vec![("name".into(), "{}".into())].into_iter().collect(),
                                    )]
                                    .into_iter()
                                    .collect(),
                                ),
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        ),
        V::Mapping(
            vec![
                ("apiVersion".into(), "v1".into()),
                ("kind".into(), "Service".into()),
                (
                    "metadata".into(),
                    V::Mapping(
                        vec![
                            ("name".into(), format!("{}-svc", &app.name.0).into()),
                            ("namespace".into(), (&config.cluster_name).into()),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                ),
                (
                    "spec".into(),
                    V::Mapping(
                        vec![
                            ("type".into(), "LoadBalancer".into()),
                            (
                                "selector".into(),
                                V::Mapping(
                                    vec![("app".into(), (&app.name).into())]
                                        .into_iter()
                                        .collect(),
                                ),
                            ), // TODO: PORTS
                        ]
                        .into_iter()
                        .collect(),
                    ),
                ),
            ]
            .into_iter()
            .collect(),
        ),
    ]
}

impl From<&Name> for V {
    fn from(value: &Name) -> Self {
        V::String(value.0.clone())
    }
}

impl From<&Image> for V {
    fn from(value: &Image) -> Self {
        V::String(value.0.clone())
    }
}

impl From<&Directory> for V {
    fn from(value: &Directory) -> Self {
        V::String(value.0.clone())
    }
}

impl From<&Environment> for V {
    fn from(value: &Environment) -> Self {
        V::Sequence(
            value
                .0
                .clone()
                .into_iter()
                .map(|(k, v)| {
                    V::Mapping(
                        vec![(
                            (&k).into(),
                            match v {
                                EnvironmentValue::Static(s) => V::String(s.clone()),
                            }
                            .into(),
                        )]
                        .into_iter()
                        .collect(),
                    )
                })
                .collect(),
        )
    }
}

impl From<&Command> for V {
    fn from(value: &Command) -> Self {
        V::Sequence(value.0.clone().into_iter().map(|arg| arg.into()).collect())
    }
}

impl<'c> From<ResourceContext<'c>> for Vec<V> {
    fn from(value: ResourceContext<'c>) -> Self {
        match value.resource {
            Resource::PostgreSQL(pgsql) => vec![from_postgres(pgsql)],
            Resource::RabbitMQ(rmq) => vec![from_rabbitmq(rmq)],
            Resource::StatelessApplication(app) => from_stateless_application(app, &value.config),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn usage() -> anyhow::Result<()> {
        let log_dir = Directory::new("/var/log".into());
        let naming_convention = NamingConvention::NsRoleResourceId { id_len: 6 };
        let cluster = Cluster::new(
            ClusterConfig::new("example".to_string().into(), naming_convention),
            vec![
                // Resource::PostgreSQL(PostgreSQL),
                // Resource::RabbitMQ(RabbitMQ),
                Resource::StatelessApplication(StatelessApplication::new(
                    "flagship/example-api-v1".to_string().into(),
                    vec!["/usr/bin/app".to_string()].into(),
                    "example-api".to_string().into(),
                    vec![NetworkPort::tcp(8080)],
                    log_dir.clone(),
                    Environment::new(vec![]),
                    3,
                )),
                Resource::StatelessApplication(StatelessApplication::new(
                    "flagship/example-queue-v1".to_string().into(),
                    vec!["/usr/bin/queue".to_string()].into(),
                    "example-queue".to_string().into(),
                    vec![],
                    log_dir.clone(),
                    Environment::new(vec![]),
                    3,
                )),
                Resource::StatelessApplication(StatelessApplication::new(
                    "flagship/example-web-v1".to_string().into(),
                    vec!["/usr/bin/httpd".to_string()].into(),
                    "example-web".to_string().into(),
                    vec![],
                    log_dir.clone(),
                    Environment::new(vec![]),
                    3,
                )),
            ],
        );

        let yaml: Vec<serde_yaml::Value> = (&cluster).into();
        let yaml = serde_yaml::to_string( &yaml)?;
        println!("{yaml}");

        Ok(())
    }
}
