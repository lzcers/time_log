use crate::{app::TimerStatus, utils::get_datetime};

pub fn display_current_timer_status(status: &TimerStatus) {
    let start_datetime = get_datetime(status.start_time);
    let date_str = start_datetime.format("%Y-%m-%d").to_string();
    let start_str = start_datetime.format("%H:%M:%S").to_string();
    let end_str = status.end_time.map_or("None".to_string(), |end| {
        get_datetime(end).format("%H:%M:%S").to_string()
    });

    let empty = "".to_string();
    let desc = status.desc.as_ref().unwrap_or(&empty);
    let duration = status.end_time.map_or_else(
        || get_datetime(chrono::Utc::now().timestamp() as u64) - start_datetime,
        |end| get_datetime(end) - start_datetime,
    );
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;
    let duration_str = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);

    // ----Current Timer--------------------------------
    // Date          Start        End        Duration
    // 2025/02/13    15:46:51  -  17:03:50   1:16:59
    // -------------------------------------------------
    // > improved display functionality
    println!("----Current Timer--------------------------------");
    println!("Date          Start        End        Duration");
    println!(
        "{}    {}  -  {}   {}",
        date_str, start_str, end_str, duration_str
    );
    println!("-------------------------------------------------");
    println!("> {}", desc);
}

// 按 tag 筛选 timer 记录
pub fn display_timer_sheet(tag: Vec<String>) {}
