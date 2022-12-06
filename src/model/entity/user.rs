use std::time::Duration;

use crate::{
    db::{get_db, get_db_tx},
    transaction,
};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub cred: String,
}
crud!(User {});

#[tokio::test]
pub async fn test() {
    let mut db_clone = get_db();
    let mut new_user = User::default();
    new_user.email = String::from("1156988195@11.com");
    new_user.username = String::from("fanqingwei");
    new_user.cred = String::from("test2");
    let tx = db_clone.acquire_begin().await.unwrap();
    let mut tx = tx.defer_async(|mut tx| async move {
        if !tx.done {
            tx.rollback().await.unwrap();
            println!("rollback");
        }
    });
    //tx.exec("select 1", vec![]).await.unwrap();
    User::insert(&mut tx, &new_user).await.unwrap();
    // tx.commit().await.unwrap();
    println!("yes forget commit");
    // drop(tx);
    //call sleep make sure tokio runtime not exit!
    // sleep(Duration::from_secs(1)).await;
}

#[tokio::test]
pub async fn test_2() {
    let mut tx = get_db_tx().await;
    let mut new_user = User::default();
    new_user.email = String::from("1156988195@11.com");
    new_user.username = String::from("fanqingwei");
    new_user.cred = String::from("test2");
    User::insert(&mut tx, &new_user).await.unwrap();
    tx.commit().await.unwrap();
    // panic!("error")
}

impl User {}
