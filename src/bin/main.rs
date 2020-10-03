use rustractor::MemoryThreadStore;
use rustractor::Thread;
use rustractor::Message;

fn main() {
    let mut thread = MemoryThreadStore::new();

    let message = Message::new(String::from("hey ma gueule ?"), String::from("Bernard"));
    thread.add(&message);

    let mut message = Message::new(String::from("sa va ?"), String::from("Bernard"));

    // Add subthread
    let mut thread2 = MemoryThreadStore::new();
    let message2 = Message::new(String::from("ou bien ?"), String::from("Bernard"));
    thread2.add(&message2);
    message.add_thread(&thread2);

    thread.add(&message);
}
