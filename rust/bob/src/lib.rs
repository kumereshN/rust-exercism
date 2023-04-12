pub fn reply(message: &str) -> &str {
    let msg = message.trim_end();
    if msg.is_empty() {
        return "Fine. Be that way!";
    }

    let is_questioning = msg.ends_with('?');
    let is_yelling =
        msg.chars().any(|c| c.is_alphabetic() && msg == msg.to_uppercase());

    match (is_yelling, is_questioning) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, _) => "Whoa, chill out!",
        (_, true) => "Sure.",
        _ => "Whatever.",
    }
}
