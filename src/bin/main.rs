use std::env;
use rustractor::ThreadStore;
use rustractor::MemoryThreadStore;
use rustractor::Message;
use rustractor::SlackReader;

fn main() {
    let slack_token = env::var("SLACK_TOKEN").expect("A SLACK_TOKEN environment variable must be defined.");
    let slack_channel = env::var("SLACK_CHANNEL").expect("A SLACK_CHANNEL environment variable must be defined.");
    let slack_reader = SlackReader::new(slack_token, slack_channel);

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
