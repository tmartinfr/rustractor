use std::env;
use rustractor::{ThreadStore, MemoryThreadStore};
use rustractor::reader::slack;
use rustractor::writer::stdout;

fn main() {
    let mut thread = MemoryThreadStore::new();

    let slack_token = env::var("SLACK_TOKEN").expect("SLACK_TOKEN environment variable must be defined.");
    let slack_conv = env::var("SLACK_CONV").expect("SLACK_CONV environment variable must be defined.");
    slack::SlackReader::read(&mut thread, &slack_conv, &slack_token);

    stdout::StdoutWriter::write(&thread);
}
