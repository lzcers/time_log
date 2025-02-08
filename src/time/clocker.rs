use std::{
    thread,
    time::{Duration, Instant},
};

// 一个计时器
// 应用可以在任何时间创建一个计时器，并将其关联到一个任务上
pub struct Clocker {
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub elapsed: u64,
}

impl Clocker {
    pub fn new() -> Self {
        Clocker {
            start_time: None,
            end_time: None,
            elapsed: 0,
        }
    }
    pub fn start(&mut self) {
        let timer = Instant::now();
        self.start_time = Some(timer.elapsed().as_millis() as u64);
        loop {
            // 每隔 100 毫秒更新一下计时器
            thread::sleep(Duration::from_millis(100));
            self.elapsed = timer.elapsed().as_millis() as u64;
            // println!("timer elapsed: {}", self.elapsed());
            if self.end_time.is_some() {
                break;
            }
        }
    }
    pub fn stop(&mut self) {
        let start_time = self.start_time.unwrap();
        let end_time = start_time + self.elapsed;
        self.end_time = Some(end_time);
        println!(
            "clock stop, start_time: {} end_time: {} elapsed: {}",
            start_time, end_time, self.elapsed
        );
    }

    pub fn elapsed(&self) -> u64 {
        self.elapsed
    }

    pub fn is_stop(&self) -> bool {
        self.end_time.is_some()
    }
}
