use rustractor::MemoryThreadStore;
use rustractor::Thread;
use rustractor::Message;

fn main() {
    let thread = MemoryThreadStore::new();
    let message = Message::new(String::from("hey ma gueule ?"), String::from("Bernard"));
    thread.add(message);
}
