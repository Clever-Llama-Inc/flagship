use std::collections::HashMap;

use flagship::recipes::*;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let stack = Stack::builder("example", false, Environment::Ephemeral(env!("USER").to_string()))
        .with_resource(Resource::PosgreSQL(PostgreSQL::new("pgvector".to_string())))
        .with_resource(Resource::RabbitMQ(RabbitMQ::new("rabbitmq".to_string())))
        .with_resource(Resource::Nginx(Nginx::new("nginx".into(), 3)))
        .with_resource(Resource::Microservice(Microservice::new(
            "my-api".into(),
            "v1".into(),
            3,
            "api".into(),
            HashMap::default(),
            vec![MicroservicePort::TCP { port: 8080, name: Some("web".to_string()) }],
        )))
        .with_resource(Resource::Microservice(Microservice::new(
            "my-queue-consumer".into(),
            "v1".into(),
            3,
            "qc".into(),
            HashMap::default(),
            Vec::default(),
        )))
        .build();

    let values = stack.as_k8s()?;
    let yaml: Vec<String> = values.iter().map(serde_yaml::to_string).try_collect()?;
    let yaml = yaml.join("\n---\n");

    println!("{yaml}");
    Ok(())
}
