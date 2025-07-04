use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Mutex;
use crate::memory::memory_trait::Memory;
use crate::model::model_trait::Message;

pub struct InMemoryMemory {
    store: Mutex<Vec<Message>>,
    kv: Mutex<HashMap<String, String>>,
}

impl InMemoryMemory {
    pub fn new() -> Self {
        Self {
            store: Mutex::new(vec![]),
            kv: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl Memory for InMemoryMemory {
    async fn recall(&self, key: &str) -> Option<String> {
        self.kv.lock().unwrap().get(key).cloned()
    }

    async fn store(&self, role: &str, content: &str) {
        let mut messages = self.store.lock().unwrap();
        messages.push(Message {
            role: role.to_string(),
            content: content.to_string(),
        });
        self.kv.lock().unwrap().insert(role.to_string(), content.to_string());
    }

    async fn load(&self) -> Vec<Message> {
        self.store.lock().unwrap().clone()
    }

    async fn clear(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.store.lock().unwrap().clear();
        self.kv.lock().unwrap().clear();
        Ok(())
    }
}
