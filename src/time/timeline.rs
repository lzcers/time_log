use super::time_slice::TimeSlice;

// 时间线即由时间切片组成的有序列表
pub struct Timeline {
    pub list: Vec<TimeSlice>,
}

impl Timeline {
    pub fn new() -> Self {
        return Timeline { list: vec![] };
    }

    // 向时间线中添加一个时间切片，保持时间线的有序性
    pub fn add(&mut self, time_slice: TimeSlice) {
        let pos = self
            .list
            .binary_search_by(|slice| slice.start_time.cmp(&time_slice.start_time))
            .unwrap_or_else(|pos| pos);
        self.list.insert(pos, time_slice);
    }

    pub fn remove(&mut self, ind: u64) {
        self.list.remove(ind as usize);
    }

    // 在时间线尾部插入一个时间切片
    pub fn push(&mut self, time_slice: TimeSlice) {
        self.list.push(time_slice);
    }
}
