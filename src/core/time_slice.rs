// 一个时间切片，代表一个时间段

#[derive(Debug, Clone)]
pub struct TimeSlice {
    pub id: u64,
    pub start_time: u64,
    pub end_time: Option<u64>,
}

impl TimeSlice {
    pub fn new(id: u64, start_time: u64, end_time: Option<u64>) -> Self {
        Self {
            id,
            start_time,
            end_time,
        }
    }

    // 获取切片的长度，单位为毫秒
    pub fn get_len(&self) -> u64 {
        if let Some(end) = self.end_time {
            end - self.start_time
        } else {
            0
        }
    }
}
