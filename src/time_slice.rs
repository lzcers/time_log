// 一个时间切片，代表一个时间段
pub struct TimeSlice {
    pub id: u64,
    pub start_time: u64,
    pub end_time: u64,
}

impl TimeSlice {
    pub fn new(id: u64, start_time: u64, end_time: u64) -> Self {
        Self {
            id,
            start_time,
            end_time,
        }
    }

    // 获取切片的长度，单位为毫秒
    pub fn get_len(&self) -> u64 {
        self.end_time - self.start_time
    }
}
