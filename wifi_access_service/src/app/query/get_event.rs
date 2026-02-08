use std::sync::{Arc, Mutex};

pub fn get_queue(event_queue: Arc<Mutex<Vec<String>>>) -> Vec<String> {
    let q = event_queue.lock().unwrap();
    q.clone()
}
