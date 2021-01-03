pub mod reader;
pub mod writer;
use std::collections::HashMap;

pub type ResultStrErr<T> = std::result::Result<T, &'static str>;

pub struct Message {
    pub content: String,
    pub author: String,
    pub timestamp: u32,
    pub thread: Option<Box<dyn ThreadStore>>,
}

impl Message {
    pub fn new(content: &str, author: &str, timestamp: u32) -> Self {
        Self {
            content: String::from(content),
            author: String::from(author),
            timestamp,
            thread: None,
        }
    }

    pub fn add_thread(&mut self, thread: Box<dyn ThreadStore>) {
        self.thread = Some(thread);
    }
}

pub trait ThreadStore {
    fn new() -> Box<Self>
    where
        Self: Sized;
    fn add_message(&mut self, message: Message);
    fn get_messages(&self) -> &Vec<Message>;
}

pub struct MemoryThreadStore {
    messages: Vec<Message>,
}

impl ThreadStore for MemoryThreadStore {
    fn new() -> Box<MemoryThreadStore> {
        let messages = Vec::new();
        Box::new(MemoryThreadStore { messages })
    }
    fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }
    fn get_messages(&self) -> &Vec<Message> {
        &self.messages
    }
}

fn reverse(h: &HashMap<String, String>) -> HashMap<String, String> {
    let mut new = HashMap::new();
    for (k, v) in h {
        new.entry(v.to_string()).or_insert(k.to_string());
    }
    new
}
