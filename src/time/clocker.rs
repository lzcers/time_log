use std::time::{SystemTime, UNIX_EPOCH};

// 一个计时器
// 应用可以在任何时间创建一个计时器，并将其关联到一个任务上
pub struct Clocker {
    start_time: Option<u64>,
    end_time: Option<u64>,
}

impl Clocker {
    pub fn new() -> Self {
        Clocker {
            start_time: None,
            end_time: None,
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Self::get_current_timestamp());
    }

    pub fn stop(&mut self) {
        let start_time = self.get_start_time();
        let end_time = Self::get_current_timestamp();
        let elapsed = end_time - start_time;
        self.end_time = Some(end_time);
        println!(
            "clock stop, start_time: {} end_time: {} elapsed: {}",
            start_time, end_time, elapsed
        );
    }

    pub fn get_start_time(&self) -> u64 {
        self.start_time.expect("The clocker is not running")
    }

    pub fn get_end_time(&self) -> Option<u64> {
        self.end_time
    }

    pub fn elapsed(&self) -> u64 {
        if self.is_stop() {
            self.get_end_time()
                .expect("The clocker was stopped, but get end_time failed!")
                - self.get_start_time()
        } else {
            Self::get_current_timestamp() - self.get_start_time()
        }
    }

    pub fn is_stop(&self) -> bool {
        self.end_time.is_some()
    }

    fn get_current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("The clocker get start time error")
            .as_millis() as u64
    }
}
