pub struct Message<'a> {
    content: String,
    author: String,
    thread: Option<&'a dyn Thread<'a>>,
}

impl<'a> Message<'a> {
    pub fn new(content: String, author: String) -> Self {
        Self { content, author, thread: None }
    }

    pub fn add_thread(&mut self, thread: &'a dyn Thread<'a>) {
        self.thread = Some(thread);
    }
}

pub struct MemoryThreadStore<'a> {
    messages: Vec<&'a Message<'a>>,
}

pub trait Thread<'a> {
    fn add(&mut self, message: &'a Message<'a>);
}

impl<'a> Thread<'a> for MemoryThreadStore<'a> {
    fn add(&mut self, message: &'a Message<'a>) {
        self.messages.push(message);
    }
}

impl<'a> MemoryThreadStore<'a> {
    pub fn new() -> MemoryThreadStore<'a> {
        let messages = Vec::new();
        MemoryThreadStore { messages }
    }
}
