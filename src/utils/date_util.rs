use chrono::{DateTime, Days, FixedOffset, Local, Offset, TimeZone, Utc};

#[test]
pub fn test_date() {
    //获取本地时间
    let mut local_time = Local::now();
    //获取 utc 时间
    let utc = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    //时间操作 获取 Option 值 如果为空就返回 Err
    local_time = local_time.checked_add_days(Days::new(2)).expect("error");
    //创建东八区 时区
    let est = FixedOffset::east_opt(9 * 3600).unwrap();
    //时间转换为具体时区时间
    let new_time = utc.with_timezone(&est);
    //获取当前毫秒
    let millis = local_time.timestamp_millis();
    println!("Local Time: {} EAT", local_time);
    println!("Local Time: {} offset", local_time.offset());
    println!("UTC Time now: {}", utc);
    println!("EST Time Now: {}", est);
    println!("New Time: {}", new_time);
    println!("Millis: {}", millis);
}
