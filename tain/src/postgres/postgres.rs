use std::collections::HashMap;

use anyhow::{anyhow, Result};
use dotenvy::vars;

use crate::{ContainerConfig, Docker, Port, PostgresConfig};

pub struct Postgres {}

impl Postgres {
    fn container_name() -> Result<String> {
        let vars: HashMap<String, String> = vars().collect();

        vars.get("POSTGRES_CONTAINER_NAME")
            .ok_or(anyhow!("No POSTGRES_CONTAINER_NAME in .env"))
            .cloned()
    }

    pub fn connection_string() -> Result<String> {
        let config = PostgresConfig::from_env()?;

        let vars: HashMap<String, String> = vars().collect();

        let password = config.password.unwrap_or("postgres".to_string());
        let host = vars.get("POSTGRES_HOST").ok_or(anyhow!("No POSTGRES_HOST in .env"))?;
        let db = config.db.unwrap_or("postgres".to_string());

        Ok(format!("postgresql://postgres:{password}@{host}/{db}"))
    }

    pub fn start_env() -> Result<()> {
        if Docker::running(&Self::container_name()?)? {
            return Ok(());
        };

        Self::start(PostgresConfig::from_env()?)?;

        Ok(())
    }

    pub fn start(config: PostgresConfig) -> Result<()> {
        let container = ContainerConfig::builder()
            .name(config.container_name)
            .image("postgres:16.2-alpine")
            .port(Port {
                host:      config.port,
                container: 5432,
            });

        let mut env: HashMap<_, _> = [
            (
                "POSTGRES_PASSWORD".to_string(),
                config.password.unwrap_or("postgres".to_string()),
            ),
            (
                "POSTGRES_DB".to_string(),
                config.db.unwrap_or("postgres".to_string()),
            ),
        ]
        .into();

        let mut config = if let Some(data) = config.data {
            env.insert("PGDATA".to_string(), data.container.clone());
            container.mount(data).build()
        } else {
            container.build()
        };

        config.env = env;

        Docker::start(config)?;

        Ok(())
    }
}
