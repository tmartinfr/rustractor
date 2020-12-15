pub mod slack {
    use super::super::Message;
    use super::super::ResultStrErr;
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
        ) -> ResultStrErr<Vec<Value>> {
            let mut all_ressources: Vec<Value> = Vec::new();
            let mut next_cursor: Option<String> = None;

            loop {
                let mut url = format!("https://slack.com/api/{}?{}", endpoint, param);

                if let Some(ref cursor) = next_cursor {
                    url = format!("{}&cursor={}", url, cursor);
                }

                let resp = ureq::get(url.as_str())
                    .set("Authorization", format!("Bearer {}", slack_token).as_str())
                    .call();

                if !resp.ok() {
                    return Err("Network error");
                }

                let content = match resp.into_string() {
                    Ok(content) => content,
                    Err(_) => return Err("Cannot read response content"),
                };
                log::trace!("Slack HTTP response : {:?}", content);

                let payload: Value = match serde_json::from_str(&content) {
                    Ok(payload) => payload,
                    Err(_) => return Err("Cannot parse Slack JSON"),
                };

                match payload.get("ok") {
                    Some(status) => match status {
                        Value::Bool(true) => (),
                        _ => return Err("Slack ok status error"),
                    },
                    None => return Err("Cannot read Slack ok status"),
                };

                match payload.get(resource_key) {
                    Some(Value::Array(ressources)) => {
                        for ressource in ressources {
                            // TODO yield these values when generator will be
                            // supported out of nightly
                            all_ressources.push(ressource.to_owned());
                        }
                    }
                    _ => return Err("Cannot read needed ressource in Slack response"),
                };

                match payload.get("response_metadata") {
                    Some(metadata) => match metadata.get("next_cursor") {
                        Some(cursor) => {
                            let cursor = match cursor {
                                Value::String(cursor) => cursor,
                                _ => return Err("Cursor is not a string"),
                            };

                            if cursor.len() > 0 {
                                log::info!("Paginating to cursor {}", cursor.to_string());
                                next_cursor = Some(cursor.to_string());
                            } else {
                                break;
                            }
                        }
                        None => break,
                    },
                    None => break,
                };
            }
            Ok(all_ressources)
        }

        fn get_conv_info(slack_conv: &String) -> ResultStrErr<(&str, &str)> {
            let vec: Vec<&str> = slack_conv.split(":").collect::<Vec<&str>>();
            match vec[0] {
                "public_channel" | "private_channel" | "im" | "mpim" => Ok((vec[0], vec[1])),
                _ => return Err("Invalid conversation type"),
            }
        }

        fn get_id_from_channels(channels: Vec<Value>, conv_name: &str) -> ResultStrErr<String> {
            log::trace!("Looking for channel ID among {:?}", channels);
            for channel in channels {
                let channel_id = match channel.get("id") {
                    Some(Value::String(id)) => id,
                    _ => return Err("Cannot read id from channel"),
                };

                let channel_name = match channel.get("name") {
                    Some(Value::String(name)) => name,
                    _ => return Err("Cannot read name from channel"),
                };

                if channel_name == conv_name {
                    return Ok(channel_id.to_string());
                }
            }
            Err("Channel id not found")
        }

        fn get_conv_id(slack_conv: &String, slack_token: &String) -> ResultStrErr<String> {
            let (conv_type, conv_name) = Self::get_conv_info(slack_conv)?;
            let channels = Self::slack_get(
                "conversations.list",
                format!("types={}", conv_type).as_str(),
                "channels",
                slack_token,
            )?;
            let conv_id = Self::get_id_from_channels(channels, conv_name)?;
            Ok(conv_id)
        }

        fn fill_thread<T: ThreadStore + 'static>(
            thread: &mut Box<T>,
            conv_id: &String,
            slack_token: &String,
        ) -> ResultStrErr<()> {
            let mut slack_messages = Self::slack_get(
                "conversations.history",
                format!("channel={}", conv_id).as_str(),
                "messages",
                slack_token,
            )?;
            slack_messages.reverse(); // Store the most recent message last
            for slack_message in slack_messages {
                let timestamp_ms = match slack_message.get("ts") {
                    Some(Value::String(ts)) => ts,
                    _ => return Err("Cannot read timestamp from Slack message"),
                };

                let timestamp = match timestamp_ms.split(".").next() {
                    Some(result) => match result.parse::<u32>() {
                        Ok(result) => result,
                        Err(_) => return Err("Cannot convert timestamp"),
                    },
                    None => return Err("Cannot parse timestamp"),
                };

                let content = match slack_message.get("text") {
                    Some(Value::String(content)) => content,
                    _ => return Err("Cannot read message content from Slack message"),
                };

                let author = match slack_message.get("user") {
                    Some(Value::String(author)) => author,
                    _ => return Err("Cannot read author from Slack message"),
                };

                let message = Message::new(content, author, timestamp);
                thread.add_message(message);
            }
            Ok(())
        }

        pub fn read<T: ThreadStore + 'static>(
            thread: &mut Box<T>,
            slack_conv: &String,
            slack_token: &String,
        ) -> ResultStrErr<()> {
            let conv_id = Self::get_conv_id(slack_conv, slack_token)?;
            Self::fill_thread(thread, &conv_id, slack_token)?;
            Ok(())
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
            assert_eq!(conv_id.unwrap(), "CT43X1ZLK");
        }
    }
}
