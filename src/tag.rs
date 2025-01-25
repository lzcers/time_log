use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: u64,
    pub name: String,
    pub color: Option<String>,
}

impl Tag {
    pub fn find_or_create(
        db: &crate::database::Database,
        name: &str,
    ) -> Result<Self, rusqlite::Error> {
        db.conn
            .execute("INSERT OR IGNORE INTO tags (name) VALUES (?1)", [name])?;

        let mut stmt = db
            .conn
            .prepare("SELECT id, name FROM tags WHERE name = ?1")?;
        let tag = stmt.query_row([name], |row| {
            Ok(Self {
                id: row.get(0)?,
                name: row.get(1)?,
                color: None,
            })
        })?;

        Ok(tag)
    }
    pub fn new(name: &str, color: Option<&str>) -> Self {
        Self {
            id: 0,
            name: name.to_string(),
            color: color.map(|c| c.to_string()),
        }
    }
}
