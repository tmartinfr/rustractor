pub mod slack {
    use super::super::Message;
    use super::super::ThreadStore;
    use serde_json::Value;
    use ureq;

    pub struct SlackReader {}

    impl SlackReader {
        fn slack_get<'a>(
            endpoint: &str,
            param: &str,
            resource_key: &str,
            slack_token: &String,
        ) -> Vec<Value> {
            let mut all_ressources: Vec<Value> = Vec::new();
            let mut next_cursor: Option<String> = None;

            loop {
                let mut url = format!("https://slack.com/api/{}?{}", endpoint, param);
                if let Some(ref some_cursor) = next_cursor {
                    url = format!("{}&cursor={}", url, some_cursor);
                }
                let resp = ureq::get(url.as_str())
                    .set("Authorization", format!("Bearer {}", slack_token).as_str())
                    .call();
                if resp.ok() {
                    if let Ok(content) = resp.into_string() {
                        log::trace!("Slack HTTP response : {:?}", content);
                        let _payload: Result<Value, _> = serde_json::from_str(&content);
                        if let Ok(payload) = _payload {
                            if let Some(ok_status) = payload.get("ok") {
                                match ok_status {
                                    Value::Bool(true) => (),
                                    _ => panic!("Slack ok status error"),
                                }
                                if let Some(ressources) = payload.get(resource_key) {
                                    if let Value::Array(ressources_array) = ressources {
                                        for ressource in ressources_array {
                                            // TODO yield these values when generator will be
                                            // supported out of nightly
                                            all_ressources.push(ressource.to_owned());
                                        }
                                    } else {
                                        panic!("Not array");
                                    }
                                    if let Some(response_metadata) =
                                        payload.get("response_metadata")
                                    {
                                        if let Some(_c) = response_metadata.get("next_cursor") {
                                            if let Value::String(c_str) = _c {
                                                if c_str.len() > 0 {
                                                    log::info!(
                                                        "Paginating to cursor {}",
                                                        c_str.to_string()
                                                    );
                                                    next_cursor = Some(c_str.to_string());
                                                } else {
                                                    break;
                                                }
                                            } else {
                                                panic!("response_metadata not a string")
                                            }
                                        } else {
                                            break;
                                        }
                                    } else {
                                        break;
                                    }
                                } else {
                                    panic!("No ressource key");
                                }
                            } else {
                                panic!("No ok status in slack response");
                            }
                        } else {
                            panic!("Cannot parse Slack JSON");
                        }
                    } else {
                        panic!("Slack error");
                    }
                } else {
                    panic!("Error in Slack response.");
                }
            }
            all_ressources
        }

        fn get_conv_info(slack_conv: &String) -> (&str, &str) {
            let vec: Vec<&str> = slack_conv.split(":").collect::<Vec<&str>>();
            match vec[0] {
                "public_channel" | "private_channel" | "im" | "mpim" => (vec[0], vec[1]),
                _ => panic!("Invalid conversation type."),
            }
        }

        fn get_id_from_channels(channels: Vec<Value>, conv_name: &str) -> String {
            log::trace!("Looking for channel ID among {:?}", channels);
            for channel in channels {
                if channel["name"].as_str().unwrap() == conv_name {
                    return String::from(channel["id"].as_str().unwrap());
                }
            }
            panic!("Channel id not found");
        }

        fn get_conv_id(slack_conv: &String, slack_token: &String) -> String {
            let (conv_type, conv_name) = Self::get_conv_info(slack_conv);
            let channels = Self::slack_get(
                "conversations.list",
                format!("types={}", conv_type).as_str(),
                "channels",
                slack_token,
            );
            let conv_id = Self::get_id_from_channels(channels, conv_name);
            conv_id
        }

        fn fill_thread<T: ThreadStore + 'static>(
            thread: &mut Box<T>,
            conv_id: &String,
            slack_token: &String,
        ) {
            let messages = Self::slack_get(
                "conversations.history",
                format!("channel={}", conv_id).as_str(),
                "messages",
                slack_token,
            );
            for slack_message in slack_messages {
                let timestamp = slack_message["ts"]
                    .as_str()
                    .unwrap()
                    .split(".")
                    .next()
                    .unwrap();
                let message = Message::new(
                    slack_message["text"].as_str().unwrap(),
                    slack_message["user"].as_str().unwrap(),
                    timestamp.parse::<u32>().unwrap(),
                );
                thread.add_message(message);
            }
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

    #[cfg(test)]
    mod tests {
        use super::SlackReader;
        use serde_json::Value;
        #[test]
        fn test_get_id_from_channels() {
            let data = r#"
                {
                    "name": "general",
                    "id": "CT43X1ZLK",
                    "is_channel": "true"
                }
            "#;
            let mut channels: Vec<Value> = Vec::new();
            channels.push(serde_json::from_str(data).unwrap());
            let conv_id = SlackReader::get_id_from_channels(channels, "general");
            assert_eq!(conv_id, "CT43X1ZLK");
        }
    }
}
