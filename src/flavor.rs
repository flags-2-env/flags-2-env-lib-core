#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatabaseFlavor {
    PostgreSql,
    CockroachDb,
}

impl DatabaseFlavor {
    /// Infer flavor from a postgres URL without mutating caller state.
    pub fn from_database_url(database_url: &str) -> Self {
        match database_url.contains("cockroach") {
            true => Self::CockroachDb,
            false => Self::PostgreSql,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DatabaseFlavor;

    #[test]
    fn cockroach_is_detected_by_url_token() {
        assert_eq!(
            DatabaseFlavor::from_database_url("postgres://root@cockroach:26257/app"),
            DatabaseFlavor::CockroachDb
        );
        assert_eq!(
            DatabaseFlavor::from_database_url("postgresql://localhost/app"),
            DatabaseFlavor::PostgreSql
        );
    }
}
