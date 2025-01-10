use std::{
    thread,
    time::{Duration, Instant},
};

use crate::utils::get_current_time;

// 一个计时器
// 应用可以在任何时间创建一个计时器，并将其关联到一个任务上
pub struct Clocker {
    pub start_time: u128,
    pub end_time: Option<u128>,
    pub elapsed: u128,
}

impl Clocker {
    pub fn new() -> Self {
        Clocker {
            start_time: get_current_time(),
            end_time: None,
            elapsed: 0,
        }
    }
    pub fn start(&mut self) {
        let timer = Instant::now();
        loop {
            // 每隔 100 毫秒更新一下计时器
            thread::sleep(Duration::from_millis(100));
            self.elapsed = timer.elapsed().as_millis();
            if self.end_time.is_some() {
                break;
            }
        }
    }
    pub fn stop(&mut self) {
        self.end_time = Some(get_current_time());
    }
}
