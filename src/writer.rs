pub mod stdout {
    use super::super::ThreadStore;
    use chrono::NaiveDateTime;

    pub struct StdoutWriter {}

    impl StdoutWriter {
        pub fn write<T: ThreadStore>(thread: &Box<T>) {
            Self::output(thread, 0);
        }

        fn output<T: ThreadStore + ?Sized>(thread: &Box<T>, level: u32) {
            for message in thread.get_messages().iter() {
                for _ in 0..level {
                    print!("    ");
                }

                let timestamp =
                    NaiveDateTime::from_timestamp(message.timestamp as i64, 0).format("%F %X");

                println!("{} {}: {}", timestamp, message.author, message.content);

                if let Some(subthread) = &message.thread {
                    Self::output(subthread, level + 1);
                }
            }
        }
    }
}
