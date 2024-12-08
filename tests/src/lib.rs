#![cfg(test)]

use anyhow::{anyhow, Result};
use serial_test::serial;
use tain::{Docker, Mount, Postgres, PostgresConfig};

#[test]
#[serial]
fn test_builder() -> Result<()> {
    Docker::check_running()?;

    assert!(!Docker::running("tain_pg_test")?);

    let home = dirs::home_dir().ok_or(anyhow!("no HOME"))?;
    let home = home.to_str().unwrap();

    let source = format!("{home}/spesogon_pg");
    let pg_data = "/spesogon_pg";

    Postgres::start(
        PostgresConfig::builder()
            .container_name("tain_pg_test")
            .data(Mount {
                host:      source,
                container: pg_data.to_string(),
            })
            .build(),
    )?;

    assert!(Docker::running("tain_pg_test")?);

    Docker::rm("tain_pg_test")?;

    assert!(!Docker::running("tain_pg_test")?);

    Ok(())
}

#[test]
#[serial]
fn test_env() -> Result<()> {
    Docker::check_running()?;

    assert!(!Docker::running("tain_test_env_pg")?);

    Postgres::start_env()?;

    assert!(Docker::running("tain_test_env_pg")?);

    Docker::rm("tain_test_env_pg")?;

    assert!(!Docker::running("tain_test_env_pg")?);

    Ok(())
}

#[test]
fn connection_string() -> Result<()> {
    assert_eq!(
        Postgres::connection_string()?,
        "postgresql://postgres:tain_test_env_pg@localhost:54320/tain_test_env_pg"
    );
    Ok(())
}
