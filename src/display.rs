use crate::{app::TimerStatus, timeline::Timeline, utils::get_datetime};

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
    // 2025/02/13    15:46:51  -  17:03:50   01:16:59
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

//
pub fn display_timer_sheet(timeline: &Timeline) {
    // filter:
    // ---------------------------------------------------------------------------------------------------------
    // Date          Start        End        Duration        Tags        Description
    // 2025/02/13    15:46:51  -  17:03:50   01:16:59        code        测试效果
    // ---------------------------------------------------------------------------------------------------------
    // Total:        100days                 100hr

    println!("filter:");
    println!("---------------------------------------------------------------------------------------------------------");
    println!("Date          Start        End        Duration        Tags        Description");
    let mut total_days = 0;
    let mut total_hours = 0;
    let mut prev_date = "".to_string();
    for time_slice in &timeline.list {
        if let Ok(time_info) = timeline.get_time_info(time_slice.id) {
            let time_slice = time_info.time_slice;
            let start_datetime = get_datetime(time_slice.start_time);
            let date_str = start_datetime.format("%Y-%m-%d").to_string();
            let start_str = start_datetime.format("%H:%M:%S").to_string();
            let end_str = get_datetime(time_slice.end_time)
                .format("%H:%M:%S")
                .to_string();

            let duration = get_datetime(time_slice.end_time) - start_datetime;
            // 计算所有的 duration 总和
            // total_hours += duration.num_hours() as i32;
            let hours = duration.num_hours();
            let minutes = duration.num_minutes() % 60;
            let seconds = duration.num_seconds() % 60;
            let duration_str = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
            let tags = time_info
                .tag
                .unwrap_or_default()
                .into_iter()
                .map(|t| t.name)
                .collect::<Vec<String>>()
                .join(" ");
            let desc = time_info.desc.unwrap_or_default();
            let print_date = if date_str == prev_date {
                "".to_string()
            } else {
                total_days += 1;
                date_str.clone()
            };
            println!(
                "{:<10}    {start_str}  -  {end_str}   {duration_str}        {:<12}{}",
                print_date,
                tags,
                desc.trim()
            );
            prev_date = date_str;
        }
    }
    println!("---------------------------------------------------------------------------------------------------------");
    println!("Total:        {total_days}days                 {total_hours}hr")
}
