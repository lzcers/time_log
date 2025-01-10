// 一个时间切片，代表一个时间段
pub struct TimeSlice {
    pub id: u128,
    pub start_time: u128,
    pub end_time: u128,
}

impl TimeSlice {
    pub fn new(id: u128, start_time: u128, end_time: u128) -> Self {
        Self {
            id,
            start_time,
            end_time,
        }
    }

    // 获取切片的长度，单位为毫秒
    pub fn get_len(&self) -> u128 {
        self.end_time - self.start_time
    }
}
