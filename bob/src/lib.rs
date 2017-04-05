pub fn reply(query: &str) -> String {
    if query.ends_with('?') {
        "Sure.".to_string()
    } else if query.is_empty() {
        "Fine. Be that way!".to_string()
    } else if query.to_uppercase() == query {
        "Whoa, chill out!".to_string()
    } else {
        "Whatever.".to_string()
    }
}
