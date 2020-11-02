use ureq;


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
    fn output(&self, level: u32);
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

    fn output(&self, level: u32) {
        for message in self.messages.iter() {
            for _ in 0..level {
                print!("    ");
            }
            println!("{}: {}", message.author, message.content);
            if let Some(subthread) = &message.thread {
                subthread.output(level + 1);
            }
        }
    }
}


const SLACK_URL: &str = "https://slack.com/api";

pub struct SlackReader{}

impl SlackReader {
    pub fn read(thread: &mut Box<MemoryThreadStore>, slack_token: String, slack_channel: String) {
        println!("Connecting to {} channel", slack_channel);

        let message = Message::new("hey ma gueule ?", "Bernard");
        thread.add(message);

        let mut message = Message::new("sa va ?", "Bernard");

        // Add subthread
        let mut thread2 = MemoryThreadStore::new();
        let message2 = Message::new("ou bien ?", "Bernard");
        thread2.add(message2);
        message.add_thread(thread2);

        thread.add(message);
    }
}
