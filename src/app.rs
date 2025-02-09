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

    pub fn start_timer(&self, duration: Option<u64>, tags: Vec<String>) {
        let mut app = self.inner.lock().expect("Failed to get app lock");
        // 只有当前没有计时器在运行时才能启动新的计时器
        if let Ok(()) = app.start_timer(tags) {
            let app_inner = self.inner.clone();
            if let Some(duration) = duration {
                std::thread::spawn(move || {
                    loop {
                        {
                            // 每隔 10 毫秒检查一下当前时间是否已经超过 duration
                            std::thread::sleep(std::time::Duration::from_millis(10));
                            let mut app = app_inner.lock().unwrap();
                            let clocker = app.current_timer.as_mut().unwrap();
                            if clocker.elapsed() > duration || clocker.is_stop() {
                                let _ = app.stop_timer();
                                break;
                            }
                            println!("duration: {} elapsed: {}", duration, clocker.elapsed());
                        }
                    }
                });
            }
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

    fn start_timer(&mut self, tags: Vec<String>) -> anyhow::Result<()> {
        // 新增tags参数处理逻辑
        if self.current_timer.is_some() {
            println!("Timer is already running!");
            return Err(Error::msg("Timer is already running!"));
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
        let mut clocker = Clocker::new();
        clocker.start();
        self.current_timer = Some(clocker);
        println!("Timer started!");
        Ok(())
    }

    fn stop_timer(&mut self) -> anyhow::Result<()> {
        if let Some(timer) = &mut self.current_timer {
            timer.stop();
            // Save time slice to database
            let tag_ids = self.current_tags.iter().map(|t| t.id).collect();
            let end_time = timer
                .get_end_time()
                .expect("The timer is stopped, but end time is None");
            self.db
                .insert_time_slice(timer.get_start_time(), end_time, tag_ids)?;
            println!("Timer stopped and saved!");
        } else {
            println!("No timer is running!");
        }
        self.current_timer = None;
        self.current_tags = vec![];
        Ok(())
    }
}
