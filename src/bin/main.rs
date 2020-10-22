use rustractor::ThreadStore;
use rustractor::MemoryThreadStore;
use rustractor::Message;

fn main() {
    let mut thread = MemoryThreadStore::new();

    let message = Message::new("hey ma gueule ?", "Bernard");
    thread.add(message);

    let mut message = Message::new("sa va ?", "Bernard");

    // Add subthread
    let mut thread2 = MemoryThreadStore::new();
    let message2 = Message::new("ou bien ?", "Bernard");
    thread2.add(message2);
    message.add_thread(thread2);

    thread.add(message);

    thread.output(0);
}
