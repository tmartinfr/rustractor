use rustractor::reader::slack;
use rustractor::writer::stdout;
use rustractor::{MemoryThreadStore, ThreadStore};
use std::env;

fn main() {
    let mut thread = MemoryThreadStore::new();

    if let Ok(slack_token) = env::var("SLACK_TOKEN") {
        let slack_conv =
            env::var("SLACK_CONV").expect("SLACK_CONV environment variable must be defined.");
        slack::SlackReader::read(&mut thread, &slack_conv, &slack_token);

        stdout::StdoutWriter::write(&thread);
    } else {
        help("SLACK_TOKEN environment variable must be defined");
    }
}

fn help(error_message: &str) {
    println!(
        r#"Error: {}
Usage: rustractor <conversation_type>:<conversation_label>
Where conversation_type is public_channel, private_channel, im, or mpim."#,
        error_message
    )
}
