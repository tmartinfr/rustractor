pub mod slack {
    use super::super::reverse;
    use super::super::Message;
    use super::super::ResultStrErr;
    use super::super::ThreadStore;
    use serde_json::Value;
    use std::collections::HashMap;
    use ureq;

    pub struct SlackReader {}

    type Users = HashMap<String, String>;

    impl SlackReader {
        fn slack_get<'a>(
            endpoint: &str,
            param: Option<&str>,
            resource_key: &str,
            slack_token: &String,
        ) -> ResultStrErr<Vec<Value>> {
            let mut all_ressources: Vec<Value> = Vec::new();
            let mut next_cursor: Option<String> = None;

            loop {
                let mut url = format!("https://slack.com/api/{}", endpoint);

                if let Some(param) = param {
                    url = format!("{}?{}", url, param);
                }

                if let Some(ref cursor) = next_cursor {
                    let mut separator = "?";
                    if param.is_some() {
                        separator = "&";
                    }
                    url = format!("{}{}cursor={}", url, separator, cursor);
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

                next_cursor = match Self::get_next_cursor(payload) {
                    Ok(cursor) => match cursor {
                        Some(cursor) => Some(cursor),
                        None => break,
                    },
                    Err(e) => return Err(e),
                };
            }
            Ok(all_ressources)
        }

        fn get_next_cursor(payload: Value) -> ResultStrErr<Option<String>> {
            let metadata = match payload.get("response_metadata") {
                Some(metadata) => metadata,
                None => return Ok(None),
            };

            let cursor = match metadata.get("next_cursor") {
                Some(Value::String(cursor)) => cursor,
                _ => return Ok(None),
            };

            if cursor.len() > 0 {
                log::info!("Paginating to cursor {}", cursor.to_string());
                return Ok(Some(cursor.to_string()));
            }

            Ok(None)
        }

        fn get_conv_info<'a>(
            slack_conv: &String,
            users: &Users,
        ) -> ResultStrErr<(String, &'a str, String)> {
            let vec: Vec<&str> = slack_conv.split(":").collect::<Vec<&str>>();

            let conv_name = match vec.get(1) {
                Some(name) => name,
                None => return Err("Conversation label not specified"),
            };

            match vec[0] {
                "public_channel" | "private_channel" | "mpim" => {
                    Ok((vec[0].to_string(), "name", conv_name.to_string()))
                }
                "im" => {
                    let user_id = match users.get(*conv_name) {
                        Some(user_id) => user_id,
                        None => return Err("Cannot find username"),
                    };
                    Ok((vec[0].to_string(), "user", user_id.to_string()))
                }
                _ => return Err("Invalid conversation type"),
            }
        }

        fn get_id_from_channels(
            channels: Vec<Value>,
            lookup_key: &str,
            lookup_value: String,
        ) -> ResultStrErr<String> {
            log::trace!("Looking for channel ID among {:?}", channels);
            for channel in channels {
                let channel_id = match channel.get("id") {
                    Some(Value::String(id)) => id,
                    _ => return Err("Cannot read id from channel payload"),
                };

                let value = match channel.get(lookup_key) {
                    Some(Value::String(key)) => key,
                    _ => return Err("Cannot read key from channel payload"),
                };

                if *value == lookup_value {
                    return Ok(channel_id.to_string());
                }
            }
            Err("Channel id not found")
        }

        fn get_conv_id(
            slack_conv: &String,
            users: &Users,
            slack_token: &String,
        ) -> ResultStrErr<String> {
            let (conv_type, lookup_key, lookup_value) = Self::get_conv_info(slack_conv, users)?;
            let channels = Self::slack_get(
                "conversations.list",
                Some(format!("types={}", conv_type).as_str()),
                "channels",
                slack_token,
            )?;
            let conv_id = Self::get_id_from_channels(channels, lookup_key, lookup_value)?;
            Ok(conv_id)
        }

        fn fill_thread<T: ThreadStore + 'static>(
            thread: &mut Box<T>,
            conv_id: &String,
            users: &Users,
            slack_token: &String,
        ) -> ResultStrErr<()> {
            let users_r = reverse(users);
            let mut slack_messages = Self::slack_get(
                "conversations.history",
                Some(format!("channel={}", conv_id).as_str()),
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
                    Some(Value::String(user_id)) => match users_r.get(user_id) {
                        Some(username) => username,
                        None => return Err("Cannot find matching username"),
                    },
                    _ => return Err("Cannot read author from Slack message"),
                };

                let message = Message::new(content, author, timestamp);
                thread.add_message(message);
            }
            Ok(())
        }

        fn get_users(slack_token: &String) -> ResultStrErr<Users> {
            let users = Self::slack_get("users.list", None, "members", slack_token)?;
            let mut simple_users: Users = HashMap::new();
            for user in users {
                let user_id = match user.get("id") {
                    Some(Value::String(id)) => id,
                    _ => return Err("Cannot read user id from Slack response"),
                };
                let user_name = match user.get("name") {
                    Some(Value::String(name)) => name,
                    _ => return Err("Cannot read username from Slack response"),
                };
                simple_users.insert(user_name.to_owned(), user_id.to_owned());
            }
            Ok(simple_users)
        }

        pub fn read<T: ThreadStore + 'static>(
            thread: &mut Box<T>,
            slack_conv: &String,
            slack_token: &String,
        ) -> ResultStrErr<()> {
            let users = Self::get_users(slack_token)?;
            let conv_id = Self::get_conv_id(slack_conv, &users, slack_token)?;
            Self::fill_thread(thread, &conv_id, &users, slack_token)?;
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
