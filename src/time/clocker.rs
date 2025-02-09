use std::{
    sync::mpsc,
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

    // 启动后会堵塞当前线程
    // 如果要停止则意味着需要在其它县城调用 stop 方法
    // Clocker 如果要在两个线程执行，那么需要用 Arc<Mutex<Clocker>> 包裹
    // 如果 Clocker 已经在其中一个线程执行了，那么另一个线程是否可以拿到 Mutex 的锁？
    // 答案是不可以，因为 Mutex 会阻塞线程，所以 Clocker 会一直在一个线程执行
    // 因此，如果要在多个线程执行 Clocker，那么需要在 start 方法中创建一个新的线程，让 start 变成非堵塞的
    // 那么如何让 start 变成非堵塞的呢？
    // 现在，我们假设 start 是非堵塞的，那么上层调用方的使用逻辑会怎么样呢？
    pub fn start(&mut self) {
        let timer = Instant::now();
        self.start_time = Some(timer.elapsed().as_millis() as u64);
        let (tx, rx) = mpsc::channel();
        // self.elapsed = timer.elapsed().as_millis() as u64;
        // // println!("timer elapsed: {}", self.elapsed());
        // if self.end_time.is_some() {
        //     break;
        // }
        let handle = thread::spawn(move || loop {
            if let Err(_) = tx.send(()) {
                break;
            }
            // 每隔 100 毫秒更新一下计时器
            thread::sleep(Duration::from_millis(100));
        });
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
