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
        let mut app = self.inner.lock().unwrap();
        app.start_timer(tags);

        let app_inner = self.inner.clone();
        if let Some(duration) = duration {
            std::thread::spawn(move || {
                loop {
                    {
                        std::thread::sleep(std::time::Duration::from_millis(10));
                        let app = app_inner.lock().unwrap();
                        println!("test");
                        let clocker_ref = app.current_timer.as_ref().unwrap();

                        let mut clocker = clocker_ref.lock().unwrap();
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
    current_timer: Option<Arc<Mutex<Clocker>>>,
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
        let clocker_ref = Arc::new(Mutex::new(Clocker::new()));
        self.current_timer = Some(clocker_ref.clone());

        std::thread::spawn(move || {
            let mut clocker = clocker_ref.lock().unwrap();
            println!("Timer started!");
            clocker.start();
        });
    }

    fn stop_timer(&mut self) -> anyhow::Result<()> {
        if let Some(timer) = &mut self.current_timer {
            let mut timer = timer.lock().unwrap();
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
