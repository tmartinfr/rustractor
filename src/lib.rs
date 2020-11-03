pub mod reader;
pub mod writer;

pub struct Message {
    pub content: String,
    pub author: String,
    pub thread: Option<Box<dyn ThreadStore>>,
}

impl Message {
    pub fn new(content: &str, author: &str) -> Self {
        Self {
            content: String::from(content),
            author: String::from(author),
            thread: None,
        }
    }

    pub fn add_thread(&mut self, thread: Box<dyn ThreadStore>) {
        self.thread = Some(thread);
    }
}

pub trait ThreadStore {
    fn add(&mut self, message: Message);
}

pub struct MemoryThreadStore  {
    messages: Vec<Box<Message>>,
}

impl MemoryThreadStore {
    pub fn new() -> Box<MemoryThreadStore> {
        let messages = Vec::new();
        Box::new(MemoryThreadStore { messages })
    }
}

impl ThreadStore for MemoryThreadStore {
    fn add(&mut self, message: Message) {
        self.messages.push(Box::new(message));
    }
}
