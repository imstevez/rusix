use crate::config;
use crate::config::Config;
use crate::utils;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::io::{Error, ErrorKind, Result};

#[derive(Clone)]
pub struct Datasource {
    pub cf: Config,
    pub rw_db: Pool<ConnectionManager<PgConnection>>,
}

impl Datasource {
    pub fn new(cf: Config) -> Result<Self> {
        let rw_db = Self::create_postgres_conn_pool(&cf.rw_db)?;
        Ok(Self { cf, rw_db })
    }

    fn create_postgres_conn_pool(
        cf: &config::Database,
    ) -> Result<Pool<ConnectionManager<PgConnection>>> {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            utils::url_encode(&cf.user),
            utils::url_encode(&cf.password),
            cf.host,
            cf.port,
            cf.database
        );

        Pool::builder()
            .max_size(cf.max_connections)
            .build(ConnectionManager::<PgConnection>::new(url))
            .map_err(|err| Error::new(ErrorKind::Other, err))
    }
}
