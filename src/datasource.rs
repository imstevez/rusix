use crate::config;
use crate::config::Config;
use crate::utils;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::deadpool::Pool;
use std::io::{Error, ErrorKind, Result};

#[derive(Clone)]
pub struct Datasource {
    pub cf: Config,
    pub rw_db: deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    pub redis_cli: redis::Client,
}

impl Datasource {
    pub fn new(cf: Config) -> Result<Self> {
        let rw_db = Self::create_postgres_conn_pool(&cf.rw_db)?;
        let redis_cli = redis::Client::open("redis://:c6bfb872-49f6-48bc-858d-2aca0c020702@127.0.0.1:6379/8").map_err(|e| Error::new(ErrorKind::Other, e))?;
        Ok(Self {
            cf,
            rw_db,
            redis_cli,
        })
    }

    fn create_postgres_conn_pool(
        cf: &config::Database,
    ) -> Result<deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>> {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            utils::url_encode(&cf.user),
            utils::url_encode(&cf.password),
            cf.host,
            cf.port,
            cf.database
        );

        let config = AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(url);
        Pool::builder(config)
            .build()
            .map_err(|err| Error::new(ErrorKind::Other, err))
    }
}
