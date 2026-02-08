use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct EventQueue {
    pub queue: Arc<Mutex<Vec<String>>>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push(&self, item: String) {
        let mut q = self.queue.lock().unwrap();
        q.push(item);
    }

    pub fn pop(&self) -> Option<String> {
        let mut q = self.queue.lock().unwrap();
        if q.is_empty() { None } else { Some(q.remove(0)) }
    }
}
