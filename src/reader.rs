pub mod slack {
    use ureq;
    use super::super::Message;
    use super::super::MemoryThreadStore;
    use super::super::ThreadStore;

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
}
