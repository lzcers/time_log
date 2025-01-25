use std::time::SystemTime;

pub fn get_current_time() -> u128 {
    if let Ok(d) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        d.as_millis()
    } else {
        panic!("get current time failed!");
    }
}
