use rusqlite::{Connection, Result};

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
            "CREATE TABLE IF NOT EXISTS tags (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT UNIQUE NOT NULL,
                    color TEXT NOT NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS time_slices (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    start_time DATETIME NOT NULL,
                    end_time DATETIME NOT NULL,
                    CHECK (end_time > start_time)
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

        Ok(())
    }

    pub fn get_connection(&self) -> &Connection {
        &self.conn
    }

    pub fn insert_time_slice(&mut self, start: u64, end: u64, tag_ids: Vec<u64>) -> Result<()> {
        let tx = self.conn.transaction()?;

        // 插入时间片段
        tx.execute(
            "INSERT INTO time_slices (start_time, end_time) VALUES (?1, ?2)",
            [start, end],
        )?;

        let time_slice_id = tx.last_insert_rowid();

        // 当存在标签ID时插入关联表
        if !tag_ids.is_empty() {
            let mut stmt =
                tx.prepare("INSERT INTO time_slice_Tags (time_slice_id, tag_id) VALUES (?1, ?2)")?;

            for tag_id in tag_ids {
                stmt.execute([time_slice_id, tag_id as i64])?;
            }
        }

        tx.commit()?;
        Ok(())
    }
}
