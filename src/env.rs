use std::str::FromStr;

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
struct Env {
    database_url: Option<String>,
}

impl FromStr for Env {
    type Err = serde_envfile::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_envfile::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::Env;

    #[test]
    fn can_get_db_url() {
        let input = "DATABASE_URL=postgres://localhost:5432";

        let env: Env = input.parse().expect("input env string failed to parse");

        let expected = Env {
            database_url: Some("postgres://localhost:5432".to_string()),
        };
        assert_eq!(env, expected);
    }

    #[test]
    fn parse_no_database_url() {
        let input = "ANOTHER_VAR=foo";

        let env: Env = input.parse().expect("input env string failed to parse");

        let expected = Env { database_url: None };
        assert_eq!(env, expected);
    }
}
