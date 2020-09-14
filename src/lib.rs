pub struct Message {
    content: String,
    author: String,
}

impl Message {
    pub fn new(content: String, author: String) -> Self {
        Self { content, author }
    }
}

pub trait Thread {
    fn add(&mut self, message: Message);
}

pub struct MemoryThreadStore {
    messages: Box<Vec<Message>>,
}

impl Thread for MemoryThreadStore {
    fn add(&mut self, message: Message) {
        self.messages.push(message);
    }
}

impl MemoryThreadStore {
    pub fn new() -> MemoryThreadStore {
        let messages = Box::new(Vec::new());
        MemoryThreadStore { messages }
    }
}
