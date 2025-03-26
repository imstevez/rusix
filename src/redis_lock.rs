use redis::{Client, Script};
use std::io::{Error, ErrorKind, Result};
use tokio::sync::mpsc;
use tokio::time::{Duration, interval, sleep};

pub enum Mode {
    Read,
    Write,
}

pub async fn lock(
    cli: Client, // redis client
    mde: Mode,   // read-or-write mode
    key: String, // lock key
    exp: u64,    // lock expiration
    rti: u64,    // retry interval ms
    rto: u64,    // retry timout ms
) -> Result<String> {
    const LUA_READ_LOCK: &str = r#"if redis.call("EXISTS", KEYS[1]) > 0 then
    return 0
else
    local ct = redis.call("TIME")
    local ex = ct[1] * 1000 + ct[2] / 1000 + ARGV[2]
    return redis.call("ZADD", KEYS[1]..":r", "NX", ex, ARGV[1])
end"#;

    // 1. clear expired read lock.
    // 2. check if write lock or read lock exists, if exists return lock failed.
    // 3. set write lock.
    const LUA_WRITE_LOCK: &str = r#"if redis.call("ZCARD", KEYS[1]..":r") > 0 then
    local ct = redis.call("TIME")
    local ex = ct[1]*1000 + ct[2]/1000
    redis.call("ZREMRANGEBYSCORE",  KEYS[1]..":r", "-INF", ex)
end
if redis.call("EXISTS", KEYS[1]) > 0 or redis.call("zcard", KEYS[1]..":r") > 0  then
    return 0
else
    redis.call("SET", KEYS[1], ARGV[1], "PX", ARGV[2])
    return 1
end"#;

    let mut conn = cli
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    let script = match mde {
        Mode::Read => Script::new(LUA_READ_LOCK),
        Mode::Write => Script::new(LUA_WRITE_LOCK),
    };

    let id = uuid::Uuid::new_v4().to_string();
    let mut itv = interval(Duration::from_millis(rti));

    let (tx, mut rx) = mpsc::channel(1);
    tokio::task::spawn(async move {
        sleep(Duration::from_millis(rto)).await;
        tx.send(()).await
    });

    loop {
        let r: u8 = script
            .key(key.clone())
            .arg(id.clone())
            .arg(exp)
            .invoke_async(&mut conn)
            .await
            .map_err(|err| Error::new(ErrorKind::Other, err))?;
        if r == 1 {
            break;
        }
        tokio::select! {
            _ = itv.tick() => {}
            _ = rx.recv() => {
                return Err(Error::new(ErrorKind::TimedOut, "retry timeout".to_string()))
            }
        }
    }
    Ok(id)
}

pub async fn unlock(cli: Client, mde: Mode, key: String, id: String) -> Result<()> {
    const LUA_READ_UNLOCK: &str = r#"return redis.call("ZREM", KEYS[1]..":r", ARGV[1])"#;

    const LUA_WRITE_UNLOCK: &str = r#"if redis.call("GET", KEYS[1]) ~= ARGV[1] then
return 0
else
return redis.call("DEL", KEYS[1])
end"#;

    let mut conn = cli
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    let script = match mde {
        Mode::Read => Script::new(LUA_READ_UNLOCK),
        Mode::Write => Script::new(LUA_WRITE_UNLOCK),
    };

    let r: u8 = script
        .key(key)
        .arg(id)
        .invoke_async(&mut conn)
        .await
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    if r == 0 {
        return Err(Error::new(
            ErrorKind::NotFound,
            "key or id not found".to_string(),
        ));
    }

    Ok(())
}

pub async fn extend(client: Client, mode: Mode, key: String, id: String, exp: u64) -> Result<()> {
    const LUA_READ_EXTEND: &str = r#"if redis.call("ZSCORE", KEYS[1]..":r", ARGV[1]) == false then
    return 0
else
    local ct = redis.call("TIME")
    local ex = ct[1] * 1000 + ct[2] / 1000 + ARGV[2]
    redis.call("ZADD", KEYS[1]..":r", ex, ARGV[1])
    return 1
end"#;

    const LUA_WRITE_EXTEND: &str = r#"if redis.call("GET", KEYS[1]) ~= ARGV[1] then
    return 0
else
    return redis.call("PEXPIRE", KEYS[1], ARGV[2])
end"#;

    let mut conn = client
        .get_multiplexed_tokio_connection()
        .await
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    let script = match mode {
        Mode::Read => Script::new(LUA_READ_EXTEND),
        Mode::Write => Script::new(LUA_WRITE_EXTEND),
    };

    let r: u8 = script
        .key(key)
        .arg(id)
        .arg(exp)
        .invoke_async(&mut conn)
        .await
        .map_err(|err| Error::new(ErrorKind::Other, err))?;

    if r == 0 {
        return Err(Error::new(
            ErrorKind::NotFound,
            "unlock retry timeout".to_string(),
        ));
    }

    Ok(())
}

pub async fn r_lock(cli: Client, key: String, exp: u64, rti: u64, rto: u64) -> Result<String> {
    lock(cli, Mode::Read, key, exp, rti, rto).await
}

pub async fn w_lock(cli: Client, key: String, exp: u64, rti: u64, rto: u64) -> Result<String> {
    lock(cli, Mode::Write, key, exp, rti, rto).await
}

pub async fn r_unlock(cli: Client, key: String, id: String) -> Result<()> {
    unlock(cli, Mode::Read, key, id).await
}

pub async fn w_unlock(cli: Client, key: String, id: String) -> Result<()> {
    unlock(cli, Mode::Write, key, id).await
}

pub async fn r_extend(cli: Client, key: String, id: String, exp: u64) -> Result<()> {
    extend(cli, Mode::Read, key, id, exp).await
}

pub async fn w_extend(cli: Client, key: String, id: String, exp: u64) -> Result<()> {
    extend(cli, Mode::Write, key, id, exp).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_write_lock() {
        let cli = redis::Client::open(
            "redis://:c6bfb872-49f6-48bc-858d-2aca0c020702@127.0.0.1:8003/0".to_string(),
        )
        .map_err(|e| Error::new(ErrorKind::Other, e))
        .unwrap();

        let id = w_lock(cli.clone(), "t_key_01".into(), 5000, 100, 2000)
            .await
            .unwrap();

        let r = w_lock(cli.clone(), "t_key_01".into(), 5000, 100, 2000).await;
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().kind(), ErrorKind::TimedOut);

        let _: () = w_unlock(cli.clone(), "t_key_01".into(), id.clone())
            .await
            .unwrap();
        let r = w_lock(cli.clone(), "t_key_01".into(), 5000, 100, 2000).await;
        assert!(r.is_ok());
    }

    #[tokio::test]
    async fn test_unlock_with_wrong_id() {
        let cli = redis::Client::open(
            "redis://:c6bfb872-49f6-48bc-858d-2aca0c020702@127.0.0.1:8003/0".to_string(),
        )
            .map_err(|e| Error::new(ErrorKind::Other, e))
            .unwrap();

        let _ = w_lock(cli.clone(), "t_key_02".into(), 5000, 100, 2000)
            .await
            .unwrap();

        let r = w_unlock(cli.clone(), "t_key_02".into(), "t_any_id".into())
            .await;

        assert!(r.is_err());
        assert_eq!(r.unwrap_err().kind(), ErrorKind::NotFound);
    }

    #[tokio::test]
    async fn test_unlock_with_wrong_key() {
        let cli = redis::Client::open(
            "redis://:c6bfb872-49f6-48bc-858d-2aca0c020702@127.0.0.1:8003/0".to_string(),
        )
            .map_err(|e| Error::new(ErrorKind::Other, e))
            .unwrap();

        let r = w_unlock(cli.clone(), "t_any_key".into(), "t_any_id".into())
            .await;

        assert!(r.is_err());
        assert_eq!(r.unwrap_err().kind(), ErrorKind::NotFound);
    }
}
