pub mod slack {
    use super::super::Message;
    use super::super::ThreadStore;
    use serde::de::DeserializeOwned;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use ureq;

    pub struct SlackReader {}

    #[derive(Serialize, Deserialize)]
    struct SlackConversationListPayload {
        channels: Vec<Value>,
    }

    #[derive(Serialize, Deserialize)]
    struct SlackConversationHistoryPayload {
        messages: Vec<Value>,
    }

    impl SlackReader {
        fn slack_get<T>(path: String, slack_token: &String) -> T
        where
            T: DeserializeOwned,
        {
            let resp = ureq::get(format!("https://slack.com/api/{}", path).as_str())
                .set("Authorization", format!("Bearer {}", slack_token).as_str())
                .call();
            // TODO paginate everything
            if resp.ok() {
                // FIXME test slack responds ok
                let content = resp.into_string().unwrap();
                serde_json::from_str(&content).unwrap()
            } else {
                panic!("Error in Slack response.");
            }
        }

        fn get_conv_info(slack_conv: &String) -> (&str, &str) {
            // TODO public_channel by default
            let vec: Vec<&str> = slack_conv.split(":").collect::<Vec<&str>>();
            match vec[0] {
                "public_channel" | "private_channel" | "im" | "mpim" => (vec[0], vec[1]),
                _ => panic!("Invalid conversation type."),
            }
        }

        fn get_id_from_channels(channels: &Vec<Value>, conv_name: &str) -> String {
            for channel in channels {
                if channel["name"].as_str().unwrap() == conv_name {
                    return String::from(channel["id"].as_str().unwrap());
                }
            }
            panic!("Channel id not found");
        }

        fn get_conv_id(slack_conv: &String, slack_token: &String) -> String {
            let (conv_type, conv_name) = Self::get_conv_info(slack_conv);
            let payload: SlackConversationListPayload = Self::slack_get(
                format!("conversations.list?types={}", conv_type),
                slack_token,
            );
            let conv_id = Self::get_id_from_channels(&payload.channels, conv_name);
            // TODO debug output
            println!("Connecting to {} conv from {}", conv_id, slack_conv);
            conv_id
        }

        fn fill_thread<T: ThreadStore + 'static>(
            thread: &mut Box<T>,
            conv_id: &String,
            slack_token: &String,
        ) {
            let payload: SlackConversationHistoryPayload = Self::slack_get(
                format!("conversations.history?channel={}", conv_id),
                slack_token,
            );
            for message in payload.messages {
                let message = Message::new(
                    message["text"].as_str().unwrap(),
                    message["user"].as_str().unwrap(),
                );
                thread.add_message(message);
            }
            // TODO handle threads : curl -F ts=1604256069.047900 -F channel= https://slack.com/api/conversations.replies
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
