use flagship::recipes::*;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let stack = Stack::builder("example", Environment::Ephemeral(env!("USER").to_string()))
        .with_resource(Resource::PosgreSQL(PostgreSQL::new(Image::new(
            "pgvector".to_string(),
            "v1".to_string(),
        ))))
        .build();

    let values = stack.as_k8s()?;

    let yaml: Vec<String> = values.iter().map(serde_yaml::to_string).try_collect()?;
    let yaml = yaml.join("\n---\n");

    println!("{yaml}");
    Ok(())
}
