use regex::Regex;

#[derive(PartialEq)]
pub enum MessageType {
    Join,
    Leave,
    Chat,
    ServerStart,
    ServerClose,
    Unknown
}

pub fn get_type(string: &str) -> MessageType {
    let join_regex = Regex::new(r"\[.*\] \[Server thread\/INFO\]: .* joined the game").unwrap();
    let leave_regex: Regex = Regex::new(r"\[.*\] \[Server thread\/INFO\]: .* lost connection: .*").unwrap();
    let chat_regex: Regex = Regex::new(r"\[.*\] \[Server thread\/INFO\]: <.*> .*").unwrap();
    let start_regex: Regex = Regex::new(r"\[.*\] \[Server thread\/INFO\]: Done").unwrap();
    let stop_regex: Regex = Regex::new(r"\[.*\] \[Server thread\/INFO\]: Stopping the server").unwrap();

    if join_regex.is_match(string) {
        return MessageType::Join;
    }

    if leave_regex.is_match(string) {
        return MessageType::Leave;
    }

    if chat_regex.is_match(string) {
        return MessageType::Chat;
    }

    if start_regex.is_match(string) {
        return MessageType::ServerStart;
    }

    if stop_regex.is_match(string) {
        return MessageType::ServerClose;
    }

    return MessageType::Unknown;
}