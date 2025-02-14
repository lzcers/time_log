use super::time_slice::TimeSlice;
use crate::tag::Tag;
use anyhow::anyhow;
use std::collections::HashMap;

// 时间线即由时间切片组成的有序列表
pub struct Timeline {
    pub list: Vec<TimeSlice>,
    // 用于记录时间切片的标签信息
    pub tags: HashMap<u64, Vec<Tag>>,
    // 用于记录时间切片的描述信息
    pub desc: HashMap<u64, String>,
}

pub struct TimeInfo {
    time_slice: TimeSlice,
    tag: Option<Vec<Tag>>,
    desc: Option<String>,
}

impl Timeline {
    pub fn new() -> Self {
        Timeline {
            list: vec![],
            tags: HashMap::new(),
            desc: HashMap::new(),
        }
    }

    pub fn init(&mut self) {}

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

    pub fn get_time_info(&self, time_slice_id: u64) -> anyhow::Result<TimeInfo> {
        let time_slice = self
            .list
            .iter()
            .find(|slice| slice.id == time_slice_id)
            .ok_or_else(|| anyhow!("Time slice not found"))?;
        let tag = self.tags.get(&time_slice_id).cloned();
        let desc = self.desc.get(&time_slice_id).cloned();
        Ok(TimeInfo {
            time_slice: time_slice.clone(),
            tag,
            desc,
        })
    }
}
