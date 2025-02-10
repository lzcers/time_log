use std::time::SystemTime;

use chrono::{DateTime, Local};

use crate::app::TimerStatus;

pub fn get_current_time() -> u64 {
    if let Ok(d) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        d.as_millis() as u64
    } else {
        panic!("get current time failed!");
    }
}

// 从毫秒单位的 unix 时间戳获取日期时间字符串
pub fn get_datetime_str(time: u64) -> String {
    let naive = DateTime::from_timestamp_millis(time as i64).expect("Invalid timestamp");
    let datetime: DateTime<Local> = DateTime::from(naive);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn display_timer_status(status: &TimerStatus) {
    let end_time = status
        .end_time
        .map_or("None".to_string(), |end| get_datetime_str(end));
    println!("   Tags: {}", status.tags.join(" "));
    println!("  Start: {}", get_datetime_str(status.start_time));
    println!("    End: {}", end_time);
    println!("Elapsed: {}s", status.elapsed / 1000);
}
