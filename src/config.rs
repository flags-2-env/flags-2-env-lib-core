#![forbid(unsafe_code)]

use crate::error::CoreError;
use crate::flavor::DatabaseFlavor;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoreConfig {
    pub database_url: String,
    pub flavor: DatabaseFlavor,
    pub read_only: bool,
}

impl CoreConfig {
    pub fn from_env() -> Result<Self, CoreError> {
        let database_url = std::env::var("FLAGS_2_ENV_DATABASE_URL")
            .map_err(|_| CoreError::InvalidDatabaseUrl)
            .and_then(require_postgres_url)?;
        Ok(Self {
            flavor: DatabaseFlavor::from_database_url(&database_url),
            read_only: is_read_only(std::env::var("FLAGS_2_ENV_DB_READ_ONLY").ok().as_deref()),
            database_url,
        })
    }
}

fn require_postgres_url(database_url: String) -> Result<String, CoreError> {
    match database_url.starts_with("postgres://") || database_url.starts_with("postgresql://") {
        true => Ok(database_url),
        false => Err(CoreError::InvalidDatabaseUrl),
    }
}

fn is_read_only(value: Option<&str>) -> bool {
    value != Some("0")
}

#[cfg(test)]
mod tests {
    use super::{is_read_only, require_postgres_url};
    use crate::error::CoreError;

    #[test]
    fn postgres_urls_are_accepted_and_others_are_rejected() {
        assert!(require_postgres_url("postgres://localhost/app".into()).is_ok());
        assert!(require_postgres_url("postgresql://localhost/app".into()).is_ok());
        assert!(matches!(
            require_postgres_url("mysql://localhost/app".into()),
            Err(CoreError::InvalidDatabaseUrl)
        ));
    }

    #[test]
    fn only_explicit_zero_disables_read_only() {
        assert!(!is_read_only(Some("0")));
        assert!(is_read_only(Some("1")));
        assert!(is_read_only(None));
    }
}
