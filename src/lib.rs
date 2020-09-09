use std::collections::BTreeMap;
use uuid::Uuid;

// trait Reader {

// }

trait Thread {
    //fn insert(msg: Message, parent: Option<u32>);
    // fn search(id: u32);
    // fn read();
}

struct Message<'a> {
    content: String,
    author: String,
    thread: Option<&'a (dyn Thread + 'a)>,
}

impl<'a> Message<'a> {
    pub fn new(content: String, author: String, thread: Option<&'a (dyn Thread + 'a)>) -> Self {
        Self { content, author, thread }
    }
}

pub struct MemoryThreadStore<'a> {
    store: Box<BTreeMap<Uuid, Message<'a>>>,
}

// impl Thread for MemoryThreadStore {

//     fn insert(msg: Message, parent_id: Option<u32>) {
//     //     if let Some(id) = parent_id {
//     //         parent_id = id;
//     //         store = self
//     //     }
//     }
// }

impl MemoryThreadStore<'_> {
    pub fn new() -> Self {
        let store =  Box::new(BTreeMap::new());
        Self { store }
    }
}
