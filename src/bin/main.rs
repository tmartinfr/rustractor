use rustractor::reader::slack;
use rustractor::writer::stdout;
use rustractor::{MemoryThreadStore, ThreadStore};
use std::env;

fn main() {
    env_logger::init();
    if let Ok(slack_token) = env::var("SLACK_TOKEN") {
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            help(Some("Invalid number of arguments"));
        }
        let first_arg = &args[1];
        if first_arg == "--help" {
            help(None);
        }
        let mut thread = MemoryThreadStore::new();
        slack::SlackReader::read(&mut thread, &first_arg, &slack_token);
        stdout::StdoutWriter::write(&thread);
    } else {
        help(Some("SLACK_TOKEN environment variable must be defined"));
    }
}

fn help(error_message: Option<&str>) {
    if let Some(msg) = error_message {
        println!("Error: {}", msg);
    }
    println!(
        r#"Usage: rustractor <conversation_type>:<conversation_label>
Where conversation_type is public_channel, private_channel, im, or mpim."#
    );
    std::process::exit(1);
}
