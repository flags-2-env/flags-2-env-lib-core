#![forbid(unsafe_code)]

use crate::config::CoreConfig;
use crate::error::CoreError;
use crate::schema::SCHEMA_REVISION;

#[derive(Clone, Debug)]
pub struct CorePool {
    pub flavor: crate::flavor::DatabaseFlavor,
    read_only: bool,
}

impl CorePool {
    pub fn connect(config: &CoreConfig) -> Result<Self, CoreError> {
        Ok(Self {
            flavor: config.flavor,
            read_only: config.read_only,
        })
    }

    pub fn assert_schema(&self, found: &str) -> Result<(), CoreError> {
        match found {
            SCHEMA_REVISION => Ok(()),
            found => Err(CoreError::SchemaRevision {
                required: SCHEMA_REVISION.to_string(),
                found: found.to_string(),
            }),
        }
    }

    pub fn migrate(&self) -> Result<(), CoreError> {
        match self.read_only {
            true => Err(CoreError::WritesDisabled),
            false => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CorePool;
    use crate::config::CoreConfig;
    use crate::error::CoreError;
    use crate::flavor::DatabaseFlavor;
    use crate::schema::SCHEMA_REVISION;

    fn pool(read_only: bool) -> CorePool {
        CorePool::connect(&CoreConfig {
            database_url: "postgres://localhost/app".into(),
            flavor: DatabaseFlavor::PostgreSql,
            read_only,
        })
        .unwrap()
    }

    #[test]
    fn schema_check_is_an_exhaustive_match() {
        assert!(pool(true).assert_schema(SCHEMA_REVISION).is_ok());
        assert!(matches!(
            pool(true).assert_schema("other"),
            Err(CoreError::SchemaRevision { .. })
        ));
    }

    #[test]
    fn migrate_is_refused_for_read_only_pools() {
        assert!(matches!(
            pool(true).migrate(),
            Err(CoreError::WritesDisabled)
        ));
        assert!(pool(false).migrate().is_ok());
    }
}
