use std::collections::HashMap;

use super::{description::Description, tag::Tag, time_slice::TimeSlice};
use anyhow::Result;
use rusqlite::Connection;

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Self::init_tables(&conn)?;
        Ok(Database { conn })
    }

    fn init_tables(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS time_slices (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    start_time DATETIME NOT NULL,
                    end_time DATETIME,
                    CHECK (end_time IS NULL OR end_time > start_time)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tags (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT UNIQUE NOT NULL,
                    color TEXT
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS time_slice_Tags  (
                    time_slice_id INTEGER NOT NULL,
                    tag_id INTEGER NOT NULL,
                    PRIMARY KEY (time_slice_id, tag_id),
                    FOREIGN KEY (time_slice_id) REFERENCES time_slices(id) ON DELETE CASCADE,
                    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // 创建一个时间片段描述表，用于存储时间片段的描述信息
        conn.execute(
            "CREATE TABLE IF NOT EXISTS time_slice_descriptions (
                    time_slice_id INTEGER PRIMARY KEY NOT NULL,
                    description TEXT,
                    FOREIGN KEY (time_slice_id) REFERENCES time_slices(id) ON DELETE CASCADE
            )",
            [],
        )?;

        Ok(())
    }

    pub fn get_all_time_slices(&self) -> Result<Vec<TimeSlice>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, start_time, end_time FROM time_slices")?;
        let time_slices = stmt
            .query_map([], |row| {
                Ok(TimeSlice {
                    id: row.get(0)?,
                    start_time: row.get(1)?,
                    end_time: row.get(2)?,
                })
            })?
            .filter_map(|result| result.ok())
            .collect();
        Ok(time_slices)
    }

    pub fn get_all_tags(&self) -> Result<Vec<Tag>> {
        let mut stmt = self.conn.prepare("SELECT id, name, color FROM tags")?;
        let tags = stmt
            .query_map([], |row| {
                Ok(Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    color: row.get(2)?,
                })
            })?
            .filter_map(|result| result.ok())
            .collect();
        Ok(tags)
    }

    pub fn get_all_descriptions(&self) -> Result<Vec<Description>> {
        let mut stmt = self
            .conn
            .prepare("SELECT time_slice_id, description FROM time_slice_descriptions")?;
        let descriptions = stmt
            .query_map([], |row| {
                Ok(Description {
                    time_slice_id: row.get(0)?,
                    description: row.get(1)?,
                })
            })?
            .filter_map(|result| result.ok())
            .collect();
        Ok(descriptions)
    }

    pub fn get_all_times_tag(&self) -> Result<HashMap<u64, Vec<Tag>>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.id, t.name, t.color, ts.time_slice_id
             FROM tags t
             JOIN time_slice_Tags ts ON t.id = ts.tag_id",
        )?;

        let mut time_slice_tags: HashMap<u64, Vec<Tag>> = HashMap::new();

        let rows = stmt.query_map([], |row| {
            Ok((
                Tag {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    color: row.get(2)?,
                },
                row.get::<_, u64>(3)?,
            ))
        })?;

        for row in rows {
            if let Ok((tag, time_slice_id)) = row {
                time_slice_tags
                    .entry(time_slice_id)
                    .or_insert_with(Vec::new)
                    .push(tag);
            }
        }

        Ok(time_slice_tags)
    }

    pub fn insert_time_slice(&mut self, start: u64, end: Option<u64>) -> Result<u64> {
        self.conn.execute(
            "INSERT INTO time_slices (start_time, end_time) VALUES (?1, ?2)",
            (start, end),
        )?;
        Ok(self.conn.last_insert_rowid() as u64)
    }

    pub fn find_or_create_tag(&self, name: &str) -> Result<Tag, rusqlite::Error> {
        self.conn
            .execute("INSERT OR IGNORE INTO tags (name) VALUES (?1)", [name])?;
        let mut stmt = self
            .conn
            .prepare("SELECT id, name FROM tags WHERE name = ?1")?;

        let tag = stmt.query_row([name], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
                color: None,
            })
        })?;

        Ok(tag)
    }

    pub fn insert_time_slice_tags(&mut self, time_slice_id: u64, tag_ids: &[u64]) -> Result<()> {
        for tag_id in tag_ids {
            self.conn.execute(
                "INSERT INTO time_slice_Tags (time_slice_id, tag_id) VALUES (?1,?2)",
                [time_slice_id, *tag_id],
            )?;
        }
        Ok(())
    }

    pub fn insert_time_slice_description(
        &mut self,
        time_slice_id: u64,
        description: &str,
    ) -> Result<()> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO time_slice_descriptions (time_slice_id, description) VALUES (?1,?2)",
        )?;
        stmt.execute((time_slice_id, description))?;
        Ok(())
    }

    pub fn insert_time_slice_info(
        &mut self,
        start: u64,
        end: Option<u64>,
        tags: &Vec<String>,
        desc: &Option<String>,
    ) -> Result<()> {
        // 插入时间片段
        let time_slice_id = self.insert_time_slice(start, end)?;

        if let Some(desc_str) = desc {
            // 写入时间片段描述
            self.insert_time_slice_description(time_slice_id, desc_str)?;

            // 写入时间片段标签
            if !tags.is_empty() {
                let mut tag_ids = vec![];
                for name in tags {
                    let tag_result = self.find_or_create_tag(&name);
                    match tag_result {
                        Ok(tag) => tag_ids.push(tag.id),
                        Err(e) => println!("Error handling tag '{}': {}", name, e),
                    }
                }
                // 当存在标签ID时插入关联表
                if !tag_ids.is_empty() {
                    self.insert_time_slice_tags(time_slice_id, &tag_ids)?;
                }
            }
        }

        Ok(())
    }

    pub fn remove_time_slice(&mut self, time_slice_id: u64) -> Result<()> {
        self.conn
            .execute("DELETE FROM time_slices WHERE id = ?1", [time_slice_id])?;
        Ok(())
    }

    pub fn update_time_slice(&mut self, time_slice: &TimeSlice) -> Result<()> {
        self.conn.execute(
            "UPDATE time_slices SET start_time =?1, end_time =?2 WHERE id =?3",
            (time_slice.start_time, time_slice.end_time, time_slice.id),
        )?;
        Ok(())
    }

    pub fn update_time_slice_tags(&mut self, time_slice_id: u64, tags: &Vec<String>) -> Result<()> {
        self.conn.execute(
            "DELETE FROM time_slice_Tags WHERE time_slice_id =?1",
            [time_slice_id],
        )?;

        let mut tag_ids = vec![];
        for name in tags {
            match self.find_or_create_tag(name) {
                Ok(tag) => {
                    tag_ids.push(tag.id);
                }
                Err(e) => println!("Error handling tag '{}': {}", name, e),
            }
        }
        if !tag_ids.is_empty() {
            self.insert_time_slice_tags(time_slice_id, &tag_ids)?;
        }
        Ok(())
    }

    pub fn update_time_slice_description(
        &mut self,
        time_slice_id: u64,
        description: &str,
    ) -> Result<()> {
        self.conn.execute(
            "UPDATE time_slice_descriptions SET description =?1 WHERE time_slice_id =?2",
            (description, time_slice_id),
        )?;
        Ok(())
    }
}
