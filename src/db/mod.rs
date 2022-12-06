use crate::config::server_config::{get_conn_string, get_redis_conn_string};
use rbatis::{executor::RBatisTxExecutorGuard, rbatis::Rbatis};
use rbdc_mysql::driver::MysqlDriver;
use redis::{Client, Commands, RedisError};

lazy_static! {
    pub static ref DB: Rbatis = {
        let rb = Rbatis::new();
        rb.init(MysqlDriver {}, &get_conn_string()).unwrap();
        rb
    };
    pub static ref Redis: Client = redis::Client::open(get_redis_conn_string()).unwrap();
}

pub fn get_db() -> Rbatis {
    DB.clone()
}

pub async fn get_db_tx() -> RBatisTxExecutorGuard {
    let mut db_clone = DB.clone();
    let tx = db_clone.acquire_begin().await.unwrap();
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            println!("事务回滚")
        }
    });
    tx
}

#[test]
fn test() -> Result<(), RedisError> {
    let mut con = Redis.get_connection()?;
    // throw away the result, just make sure it does not fail
    // let _ : () = con.set("my_key", 42)?;
    // read back the key and return it.  Because the return value
    // from the function is a result for integer this will automatically
    // convert into one.
    con.set("my_key_sags", "test")?;
    let result: Result<String, RedisError> = con.get("my_key_sags");
    let mut resultValue: String = "".into();
    match result {
        Ok(value) => {
            resultValue = value.to_string();
            println!("{}", value);
        }
        Err(e) => {
            println!("{}", e.to_string());
        }
    }
    println!("{}", resultValue);
    redis::cmd("set").arg("my_key").arg(58).execute(&mut con);
    let val: i32 = redis::cmd("GET").arg("my_key").query::<i32>(&mut con)?;
    println!("{}", val);
    Ok(())
}
