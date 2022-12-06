use crate::db::Redis;
use redis::{Commands, Connection};

fn get_conn() -> Connection {
    Redis.get_connection().expect("获取redis连接异常")
}

pub fn set(key: String, value: String) {
    let mut con = get_conn();
    con.set(key, value).expect("redis set error")
}

pub fn get(key: String) -> Option<String> {
    let mut con = get_conn();
    match con.get(key) {
        Ok(value) => Some(value),
        Err(e) => None,
    }
}

pub fn set_ex(key: String, value: String, second: usize) {
    let mut con = get_conn();
    con.set_ex(key, value, second).expect("redis set error")
}
