pub mod slack {
    use super::super::Message;
    use super::super::ThreadStore;
    use ureq;

    pub struct SlackReader {}

    impl SlackReader {
        fn slack_get(path: String, slack_token: &String) -> String {
            let resp = ureq::get(format!("https://slack.com/api/{}", path).as_str())
                .set("Authorization", format!("Bearer {}", slack_token).as_str())
                .call();
            // TODO paginate everything
            if resp.ok() {
                // FIXME test slack responds ok
                resp.into_string().unwrap()
            } else {
                panic!("Error in Slack response.");
            }
        }

        fn get_conv_info(slack_conv: &String) -> (&str, &str) {
            // TODO check conv_type
            let vec: Vec<&str> = slack_conv.split(":").collect::<Vec<&str>>();
            (vec[0], vec[1])
        }

        fn get_conv_id(slack_conv: &String, slack_token: &String) -> String {
            let (conv_type, conv_name) = Self::get_conv_info(slack_conv);
            let payload = Self::slack_get(
                format!("conversations.list?types={}", conv_type),
                slack_token,
            );
            // TODO Retrieve id from response
            let conv_id = String::from("ABC");
            // FIXME debug output
            println!("Connecting to {} conv from {}", conv_id, slack_conv);
            conv_id
        }

        fn fill_thread<T: ThreadStore + 'static>(
            thread: &mut Box<T>,
            conv_id: &String,
            slack_token: &String,
        ) {
            // curl -F channel= https://slack.com/api/conversations.history
            // curl -F ts=1604256069.047900 -F channel= https://slack.com/api/conversations.replies
            // TODO return thread
        }

        pub fn read<T: ThreadStore + 'static>(
            thread: &mut Box<T>,
            slack_conv: &String,
            slack_token: &String,
        ) {
            let conv_id = Self::get_conv_id(slack_conv, slack_token);
            Self::fill_thread(thread, &conv_id, slack_token);
        }
    }
}
