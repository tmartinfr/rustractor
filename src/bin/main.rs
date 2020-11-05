use std::env;
use rustractor::{ThreadStore, MemoryThreadStore};
use rustractor::reader::slack;
use rustractor::writer::stdout;

fn main() {
    let mut thread = MemoryThreadStore::new();

    let slack_token = env::var("SLACK_TOKEN").expect("A SLACK_TOKEN environment variable must be defined.");
    let slack_channel = env::var("SLACK_CHANNEL").expect("A SLACK_CHANNEL environment variable must be defined.");
    slack::SlackReader::read(&mut thread, slack_token, slack_channel);

    stdout::StdoutWriter::write(&thread);
}
