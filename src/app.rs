use anyhow::Error;

use crate::{database::Database, tag::Tag, time::clocker::Clocker};
use std::sync::{Arc, Mutex};

pub struct AppHandle {
    inner: Arc<Mutex<App>>,
}

impl AppHandle {
    pub fn new(app: App) -> Self {
        AppHandle {
            inner: Arc::new(Mutex::new(app)),
        }
    }

    // 现在让我们模拟 Command 调用 AppHandle 的 start_timer 方法
    // 当 start_time 调用时，可能存在两种情况
    // 1. duration 为 None，那么只是启动一个计时器，不会停止
    // 2. duration 不为 None，那么会启动一个计时器，并在 duration 时间后停止
    // 3. 如果已经存在一个计时器在运行，那么会返回一个错误
    pub fn start_timer(&self, duration: Option<u64>, tags: Vec<String>) {
        let mut app = self.inner.lock().unwrap();
        app.start_timer(tags);

        // 当函数被调用时，创建 Clocker 实例并启动计时器
        // 如果 duration 不为 None，那么会在 duration 时间后停止计时器
        // 为了实现 duration 时间后停止计时器，我们需要在 start_timer 方法中创建一个新的线程
        // 该线程会每隔 10 毫秒检查一下当前时间是否已经超过 duration
        // 如果超过了，那么会调用 stop_timer 方法停止当前的计时器
        // 显然，这里为了能在检查线程中调用 stop_timer 方法，我们需要将 App 实例的 Arc<Mutex<App>> 传递给检查线程

        let app_inner = self.inner.clone();
        if let Some(duration) = duration {
            std::thread::spawn(move || {
                loop {
                    {
                        // 每隔 10 毫秒检查一下当前时间是否已经超过 duration
                        std::thread::sleep(std::time::Duration::from_millis(10));
                        let mut app: std::sync::MutexGuard<'_, App> = app_inner.lock().unwrap();
                        let clocker = app.current_timer.as_mut().unwrap();

                        if clocker.elapsed() > duration || clocker.is_stop() {
                            if !clocker.is_stop() {
                                clocker.stop();
                            }
                            break;
                        }
                        println!("duration: {} elapsed: {}", duration, clocker.elapsed());
                    }
                }
                let mut app = app_inner.lock().unwrap();
                let _ = app.stop_timer();
            });
        }
    }

    pub fn stop_timer(&self) -> anyhow::Result<()> {
        let mut app = self.inner.lock().unwrap();
        app.stop_timer()
    }
}

pub struct App {
    db: Database,
    current_timer: Option<Clocker>,
    current_tags: Vec<Tag>,
}

impl App {
    pub fn new(db: Database) -> Self {
        App {
            db,
            current_timer: None,
            current_tags: vec![],
        }
    }

    fn start_timer(&mut self, tags: Vec<String>) {
        // 新增tags参数处理逻辑
        if self.current_timer.is_some() {
            println!("Timer is already running!");
            return;
        }

        // 处理标签
        if !tags.is_empty() {
            for name in tags {
                match Tag::find_or_create(&self.db, &name) {
                    Ok(tag) => self.current_tags.push(tag),
                    Err(e) => eprintln!("Error handling tag '{}': {}", name, e),
                }
            }
        }

        // 创建时钟实例并启动
        // 考虑多线程的场景， start_timer 会创建一个 Clocker 实例并启动，并且 Clocker.start 是非堵塞的
        let mut clocker = Clocker::new();
        clocker.start();
        self.current_timer = Some(clocker);
        println!("Timer started!");
    }

    fn stop_timer(&mut self) -> anyhow::Result<()> {
        if let Some(timer) = &mut self.current_timer {
            timer.stop();
            let Some(start_time) = timer.start_time else {
                return Err(Error::msg("Timer is not running!"));
            };
            let Some(end_time) = timer.end_time else {
                return Err(Error::msg("Timer is not stop!"));
            };
            // Save time slice to database
            let tag_ids = self.current_tags.iter().map(|t| t.id).collect();

            self.db.insert_time_slice(start_time, end_time, tag_ids)?;

            println!("Timer stopped and saved!");
        } else {
            println!("No timer is running!");
        }
        self.current_timer = None;
        self.current_tags = vec![];
        Ok(())
    }
}
