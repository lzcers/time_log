use crate::{database::Database, tag::Tag, time::clocker::Clocker};
use std::sync::{Arc, Mutex};

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

    pub fn start_timer(&mut self, duration: u64, tags: Vec<String>) {
        // 新增tags参数处理逻辑
        if self.current_timer.is_some() {
            println!("Timer is already running!");
            return;
        }

        let clocker = Arc::new(Mutex::new(Clocker::new()));
        self.current_timer = Some(clocker.clone());

        // Handle duration if provided
        let clocker = clocker.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(duration * 60));
            let mut clocker = clocker.lock().unwrap();
            clocker.stop();
        });

        // Handle tags
        if !tags.is_empty() {
            for name in tags {
                match Tag::find_or_create(&self.db, &name) {
                    Ok(tag) => self.current_tags.push(tag),
                    Err(e) => eprintln!("Error handling tag '{}': {}", name, e),
                }
            }
        }

        println!("Timer started!");
    }

    pub fn stop_timer(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(timer) = &self.current_timer {
            let mut timer = timer.lock().unwrap();
            timer.stop();

            // Save time slice to database
            self.db.insert_time_slice(
                timer.start_time,
                timer.end_time.unwrap(),
                self.current_tags.iter().map(|t| t.id).collect(),
            )?;

            println!("Timer stopped and saved!");
        } else {
            println!("No timer is running!");
        }
        self.current_timer = None;
        self.current_tags = vec![];
        Ok(())
    }
}
