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
        (Regex::new(r"\[.*\] \[Server thread\/INFO\]: .* joined the game").unwrap(),        MessageType::Join),
        (Regex::new(r"\[.*\] \[Server thread\/INFO\]: .* lost connection: .*").unwrap(),    MessageType::Leave),
        (Regex::new(r"\[.*\] \[Server thread\/INFO\]: <.*> .*").unwrap(),                   MessageType::Chat),
        (Regex::new(r"\[.*\] \[Server thread\/INFO\]: Done").unwrap(),                      MessageType::ServerStart),
        (Regex::new(r"\[.*\] \[Server thread\/INFO\]: Stopping the server").unwrap(),       MessageType::ServerClose),
        (Regex::new(r"\[.*\] \[User Authenticator #1/INFO\]: UUID of player").unwrap(),     MessageType::AddUuid)
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