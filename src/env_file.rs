use std::str::FromStr;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct EnvFile {
    pub database_url: Option<String>,
}

impl FromStr for EnvFile {
    type Err = serde_envfile::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_envfile::from_str(s)
    }
}

impl EnvFile {
    pub fn load() -> anyhow::Result<Self> {
        let mut current_dir = std::env::current_dir()?;
        loop {
            let env_file = current_dir.join(".env");
            if env_file.is_file() {
                let env = std::fs::read_to_string(env_file)?;
                return env.parse::<EnvFile>().map_err(|e| e.into());
            }
            if !current_dir.pop() {
                return Err(anyhow::anyhow!("could not find .env file"));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EnvFile;

    #[test]
    fn can_get_db_url() {
        let input = "DATABASE_URL=postgres://localhost:5432";

        let env: EnvFile = input.parse().expect("input env string failed to parse");

        let expected = EnvFile {
            database_url: Some("postgres://localhost:5432".to_string()),
        };
        assert_eq!(env, expected);
    }

    #[test]
    fn parse_no_database_url() {
        let input = "ANOTHER_VAR=foo";

        let env: EnvFile = input.parse().expect("input env string failed to parse");

        let expected = EnvFile { database_url: None };
        assert_eq!(env, expected);
    }
}
