use anyhow::Error;

use crate::{database::Database, tag::Tag, time::clocker::Clocker};
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::JoinHandle,
    time::SystemTime,
};

pub struct AppHandle {
    inner: Arc<Mutex<App>>,
    timer_handle: Option<JoinHandle<()>>,
    should_stop_flag: Arc<AtomicBool>,
}

impl AppHandle {
    pub fn new(app: App) -> Self {
        AppHandle {
            inner: Arc::new(Mutex::new(app)),
            timer_handle: None,
            should_stop_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start_timer(&mut self, duration: Option<u64>, tags: Vec<String>) -> anyhow::Result<()> {
        let mut app = self.inner.lock().expect("Failed to get app lock");
        // 只有当前没有计时器在运行时才能启动新的计时器
        if let Ok(()) = app.start_timer(tags) {
            let clocker_start_time = app
                .current_timer
                .as_ref()
                .expect("Get current timer failed.")
                .get_start_time();
            if let Some(duration) = duration {
                let app_inner_clone = self.inner.clone();
                let should_stop_flag = self.should_stop_flag.clone();
                self.timer_handle = Some(std::thread::spawn(move || loop {
                    let elapsed = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .expect("Get now time failed.")
                        .as_millis() as u64
                        - clocker_start_time;

                    if should_stop_flag.load(Ordering::Relaxed) {
                        break;
                    }
                    if elapsed > duration {
                        if let Err(e) = app_inner_clone
                            .lock()
                            .expect("Get app lock failed.")
                            .stop_timer()
                        {
                            eprintln!("Error stopping timer: {}", e);
                        }
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }));
            }
        }
        Ok(())
    }

    pub fn stop_timer(&mut self) -> anyhow::Result<()> {
        let mut app = self
            .inner
            .lock()
            .expect("App handle stop timer failed, can't get app lock");
        if self.timer_handle.is_some() {
            self.should_stop_flag.store(true, Ordering::Relaxed);
        }
        let _ = app.stop_timer();
        self.should_stop_flag.store(false, Ordering::Relaxed);
        self.timer_handle = None;
        Ok(())
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
            let end_time: u64 = timer
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
