use crate::config;
use crate::config::Config;
use crate::utils;
use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::{AsyncDieselConnectionManager, PoolableConnection};
use redis::AsyncCommands;
use std::io::{Error, ErrorKind, Result};

#[derive(Clone)]
pub struct State {
    pub cfg: Config,
    pub rw_db: deadpool::managed::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>,
    pub redis_cli: redis::Client,
}

impl State {
    pub async fn new(cfg: Config) -> Result<Self> {
        let rw_db = Self::create_postgres_conn_pool(&cfg.rw_db).await?;
        let redis_cli = Self::create_redis_cli(&cfg.redis).await?;

        Ok(Self {
            cfg,
            rw_db,
            redis_cli,
        })
    }

    async fn create_postgres_conn_pool(
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
        let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
        let pool = Pool::builder(config)
            .build()
            .map_err(|err| Error::new(ErrorKind::Other, err))?;
        let mut con = pool
            .get()
            .await
            .map_err(|err| Error::new(ErrorKind::Other, err))?;
        let _: () = con
            .ping(&diesel_async::pooled_connection::RecyclingMethod::Fast)
            .await
            .map_err(|err| Error::new(ErrorKind::Other, err))?;
        Ok(pool)
    }

    async fn create_redis_cli(cf: &config::Redis) -> Result<redis::Client> {
        let url = format!(
            "redis://:{}@{}:{}/{}",
            utils::url_encode(&cf.password),
            cf.host,
            cf.port,
            cf.database
        );
        let cli = redis::Client::open(url).map_err(|e| Error::new(ErrorKind::Other, e))?;
        let mut con = cli
            .get_multiplexed_tokio_connection()
            .await
            .map_err(|err| Error::new(ErrorKind::Other, err))?;
        let _: () = con
            .ping()
            .await
            .map_err(|e| Error::new(ErrorKind::Other, e))?;
        Ok(cli)
    }
}
