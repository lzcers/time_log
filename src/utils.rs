use std::time::SystemTime;

pub fn get_current_time() -> u64 {
    if let Ok(d) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        d.as_millis() as u64
    } else {
        panic!("get current time failed!");
    }
}
