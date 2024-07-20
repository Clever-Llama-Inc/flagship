use derive_more::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum NamingConvention {
    NsRoleResourceId { id_len: usize },
}

#[derive(Debug, Clone, Constructor, From, Into, Serialize, Deserialize)]
pub struct Name(pub String);

#[derive(Debug, Constructor, From, Into, Serialize, Deserialize)]
pub struct Image(pub String);

#[derive(Debug, Constructor, From, Into, Serialize, Deserialize)]
pub struct Command(pub Vec<String>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkProtocol {
    Tcp,
    Udp,
}

#[derive(Debug, Constructor, Serialize, Deserialize)]
pub struct NetworkPort {
    pub port: u16,
    pub protocol: NetworkProtocol,
    pub name: Option<Name>,
}

impl NetworkPort {
    pub fn tcp(port: u16) -> Self {
        Self::new(port, NetworkProtocol::Tcp, None)
    }

    pub fn udp(port: u16) -> Self {
        Self::new(port, NetworkProtocol::Udp, None)
    }

    pub fn with_name(self, name: Name) -> Self {
        Self {
            name: Some(name),
            ..self
        }
    }
}

#[derive(Debug, Constructor, From, Into, Clone, Serialize, Deserialize)]
pub struct Directory(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnvironmentValue {
    Static(String),
}

#[derive(Debug, Clone, Constructor, From, Into, Serialize, Deserialize)]
pub struct Environment(pub Vec<(Name, EnvironmentValue)>);

#[derive(Debug, Constructor, Serialize, Deserialize)]
pub struct ClusterConfig {
    pub cluster_name: Name,
    pub naming_convention: NamingConvention,
}

#[derive(Debug, Constructor, Serialize, Deserialize)]
pub struct Cluster {
    pub cluster_config: ClusterConfig,
    pub resources: Vec<Resource>,
}

#[derive(Debug, Constructor, Deserialize, Serialize)]
pub struct PostgreSQL;

#[derive(Debug, Constructor, Deserialize, Serialize)]
pub struct RabbitMQ;

#[derive(Debug, Constructor, Deserialize, Serialize)]
pub struct StatelessApplication {
    pub image: Image,
    pub command: Command,
    pub name: Name,
    pub ports: Vec<NetworkPort>,
    pub logs: Directory,
    pub environment: Environment,
    pub nodes: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Resource {
    PostgreSQL(PostgreSQL),
    RabbitMQ(RabbitMQ),
    StatelessApplication(StatelessApplication),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usage() -> anyhow::Result<()> {
        let log_dir = Directory::new("/var/log".into());
        let naming_convention = NamingConvention::NsRoleResourceId { id_len: 6 };
        let cluster = Cluster::new(
            ClusterConfig::new("example".to_string().into(), naming_convention),
            vec![
                Resource::PostgreSQL(PostgreSQL),
                Resource::RabbitMQ(RabbitMQ),
                Resource::StatelessApplication(StatelessApplication::new(
                    "flagship/example-api-v1".to_string().into(),
                    vec!["/usr/bin/api".to_string()].into(),
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

        let yaml = serde_yaml::to_string(&cluster)?;

        println!("{}", yaml);

        Ok(())
    }
}
