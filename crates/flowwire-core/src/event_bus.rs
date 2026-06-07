use std::collections::HashMap;

/// Simple event bus
pub struct EventBus {
    listeners: HashMap<String, Vec<Box<dyn Fn(String) + Send + Sync>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }

    pub fn subscribe<F>(&mut self, event: &str, callback: F)
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        self.listeners.entry(event.to_string())
            .or_insert(Vec::new())
            .push(Box::new(callback));
    }

    pub fn emit(&self, event: &str, data: String) {
        if let Some(listeners) = self.listeners.get(event) {
            for cb in listeners {
                cb(data.clone());
            }
        }
    }
}
