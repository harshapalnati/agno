use async_trait::async_trait;
use chrono::Utc;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use crate::model::model_trait::Message;
use crate::memory::memory_trait::Memory;

#[derive(Clone)]
pub struct SqliteMemory {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteMemory {
    pub fn new(path: &str) -> rusqlite::Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS memory (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                role TEXT NOT NULL,
                content TEXT NOT NULL,
                timestamp TEXT NOT NULL
            );
            "#,
        )?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
}

#[async_trait]
impl Memory for SqliteMemory {
    async fn recall(&self, key: &str) -> Option<String> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT content FROM memory WHERE content LIKE ? ORDER BY id DESC LIMIT 1"
        ).ok()?;
        let mut rows = stmt.query(params![format!("%{}%", key)]).ok()?;
        if let Some(row) = rows.next().ok()? {
            row.get(0).ok()
        } else {
            None
        }
    }

    async fn store(&self, role: &str, content: &str) {
        let conn = self.conn.lock().unwrap();
        let timestamp = Utc::now().to_rfc3339();
        let _ = conn.execute(
            "INSERT INTO memory (role, content, timestamp) VALUES (?1, ?2, ?3)",
            params![role, content, timestamp],
        );
    }

    async fn load(&self) -> Vec<Message> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn
            .prepare("SELECT role, content FROM memory ORDER BY id ASC")
            .unwrap();
        let rows = stmt
            .query_map([], |row| {
                Ok(Message {
                    role: row.get(0)?,
                    content: row.get(1)?,
                })
            })
            .unwrap();
        rows.filter_map(Result::ok).collect()
    }

    async fn clear(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM memory", [])?;
        Ok(())
    }
}
