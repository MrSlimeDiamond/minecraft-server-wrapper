use regex::Regex;

#[derive(PartialEq)]
pub enum MessageType {
    Join,
    Leave,
    Chat,
    ServerStart,
    ServerClose,
    AddUuid,
    Unknown
}

pub fn get_type(string: &str) -> MessageType {
    let types: Vec<(Regex, MessageType)> = vec![
        (Regex::new(r"INFO\]: .*\[\/.*\] logged in.*").unwrap(),    MessageType::Join),
        (Regex::new(r"INFO\]: .* lost connection: .*").unwrap(),    MessageType::Leave),
        (Regex::new(r"INFO\]: <.*> .*").unwrap(),                   MessageType::Chat),
        (Regex::new(r"INFO\]: Done").unwrap(),                      MessageType::ServerStart),
        (Regex::new(r"INFO\]: Stopping the server").unwrap(),       MessageType::ServerClose),
        (Regex::new(r"INFO\]: UUID of player").unwrap(),            MessageType::AddUuid)
    ];

    for (regex, message_type) in types {
        match regex.is_match(string) {
            true => {
                return message_type
            },
            false => {}
        }
    };

   MessageType::Unknown
}
