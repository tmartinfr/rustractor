pub mod stdout {
    use super::super::Message;
    use super::super::MemoryThreadStore;
    use super::super::ThreadStore;

    pub struct StdoutWriter{}

    impl StdoutWriter {
        pub fn write(thread: &Box<MemoryThreadStore>) {
            Self::output(thread, 0);
        }

        fn output(thread: &Box<MemoryThreadStore>, level: u32) {
            for message in thread.messages.iter() {
                for _ in 0..level {
                    print!("    ");
                }
                println!("{}: {}", message.author, message.content);
            }
        }
    }
}
