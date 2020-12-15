use rustractor::reader::slack;
use rustractor::writer::stdout;
use rustractor::{MemoryThreadStore, ThreadStore};
use std::env;

fn main() {
    env_logger::init();
    if let Ok(slack_token) = env::var("SLACK_TOKEN") {
        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            help(Some("Invalid number of arguments."), true);
        }
        let first_arg = &args[1];
        if first_arg == "--help" {
            help(None, true);
        }
        let mut thread = MemoryThreadStore::new();
        match slack::SlackReader::read(&mut thread, &first_arg, &slack_token) {
            Ok(()) => {
                stdout::StdoutWriter::write(&thread);
            }
            Err(msg) => help(Some(msg), false),
        }
    } else {
        help(
            Some("SLACK_TOKEN environment variable must be defined"),
            false,
        );
    }
}

fn help(error_message: Option<&str>, usage: bool) {
    if let Some(msg) = error_message {
        println!("{}", msg);
    }
    if usage {
        println!(
            r#"Usage: rustractor <conversation_type>:<conversation_label>
Where conversation_type is public_channel, private_channel, im, or mpim."#
        );
    }
    std::process::exit(1);
}
