use std::time::SystemTime;

use chrono::{DateTime, Local};

pub fn get_current_time() -> u64 {
    if let Ok(d) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        d.as_millis() as u64
    } else {
        panic!("get current time failed!");
    }
}

pub fn get_datetime(time: u64) -> DateTime<Local> {
    let naive = DateTime::from_timestamp_millis(time as i64).expect("Invalid timestamp");
    DateTime::from(naive)
}

// 从毫秒单位的 unix 时间戳获取日期时间字符串
pub fn get_datetime_str(time: u64) -> String {
    let naive = DateTime::from_timestamp_millis(time as i64).expect("Invalid timestamp");
    let datetime: DateTime<Local> = DateTime::from(naive);
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}
// 获取日期
pub fn get_date_str(time: u64) -> String {
    let naive = DateTime::from_timestamp_millis(time as i64).expect("Invalid timestamp");
    let datetime: DateTime<Local> = DateTime::from(naive);
    datetime.format("%Y-%m-%d").to_string()
}

pub fn parse_duration(s: &str) -> Result<u64, String> {
    if s.is_empty() {
        return Err("Duration string cannot be empty".to_string());
    }
    if let Some(last_char) = s.chars().last() {
        // 检查最后一个字符是否为数字，实则判断整个字符串是否为纯数字
        if last_char.is_digit(10) {
            // 纯数字视为秒
            if s.chars().all(|c| c.is_digit(10)) {
                return s
                    .parse::<u64>()
                    .map(|seconds| seconds * 1000)
                    .map_err(|_| format!("Invalid number: '{}'", s));
            }
        }
        if last_char.is_alphabetic() {
            // 判断是否为带单位的时间
            // 解析带单位的时间
            let (num_str, unit) = s.split_at(s.len() - last_char.len_utf8());
            let num = num_str
                .parse::<u64>()
                .map_err(|_| format!("Invalid number: '{}'", num_str))?;
            return match unit {
                "s" => Ok(num * 1000),      // 秒转毫秒
                "m" => Ok(num * 60 * 1000), // 分钟转毫秒
                _ => Err(format!(
                    "Invalid time unit: '{}'. Expected 's' or 'm'",
                    unit
                )),
            };
        }
    }
    return Err(format!("Invalid duration string '{s}'"));
}

// s 15m "#code 编写 timeLog"
// <command> [duration] [desc]
pub fn parse_start_args(args: Vec<String>) -> (Option<u64>, Option<String>) {
    if args.is_empty() {
        return (None, None);
    }
    // 尝试解析第一个参数作为持续时间
    let mut duration = None;
    let mut desc = None;

    // 先尝试将第一个参数解析为持续时间
    if let Some(first) = args.first() {
        if let Ok(d) = parse_duration(first) {
            duration = Some(d);
            // 如果还有其他参数，将剩余所有参数合并为描述
            if args.len() > 1 {
                desc = Some(args[1..].join(" "));
            }
        } else {
            // 如果第一个参数不是持续时间，则整个参数列表都作为描述
            desc = Some(args.join(" "));
        }
    }
    (duration, desc)
}

pub fn parse_tags(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .filter(|word| word.starts_with('#'))
        .map(|tag| tag.trim_start_matches('#').to_string())
        .collect()
}
