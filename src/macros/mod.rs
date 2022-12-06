use crate::{db::{get_db}};
#[macro_export]
macro_rules! transaction { 
    () => ({ 
        let mut db_clone = get_db();
        let tx = db_clone.acquire_begin().await.unwrap();
        let mut tx = tx.defer_async(|mut tx| async move {
            if !tx.done {
                tx.commit().await.unwrap();
                println!("事务提交")
            }
        });
        tx
    })
}
