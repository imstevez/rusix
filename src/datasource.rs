use crate::config;
use crate::config::Config;
use crate::utils;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use std::io;

#[derive(Clone)]
pub struct Datasource {
    pub cf: Config,
    pub rw_db: Pool<ConnectionManager<PgConnection>>,
}

impl Datasource {
    pub fn new(cf: Config) -> io::Result<Self> {
        let rw_db = Self::create_postgres_conn_pool(&cf.rw_db)?;
        Ok(Self { cf, rw_db })
    }

    fn create_postgres_conn_pool(
        cf: &config::Database,
    ) -> io::Result<Pool<ConnectionManager<PgConnection>>> {
        let url = format!(
            "postgres://{}:{}@{}:{}/{}",
            utils::url_encode(&cf.user),
            utils::url_encode(&cf.password),
            cf.host,
            cf.port,
            cf.database
        );

        let manager = ConnectionManager::<PgConnection>::new(url);

        let r = Pool::builder()
            .test_on_check_out(true)
            .max_size(cf.max_connections)
            .build(manager);

        match r {
            Ok(pool) => Ok(pool),
            Err(err) => Err(io::Error::new(io::ErrorKind::Other, err)),
        }
    }
}
