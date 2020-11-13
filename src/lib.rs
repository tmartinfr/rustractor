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
