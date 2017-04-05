use std::collections::HashMap;

pub fn word_count(s: &str) -> HashMap<String, u32> {
    let mut iter = s.split_whitespace();
    let mut map: HashMap<String, u32> = HashMap::new();
    loop {
        match iter.next() {
            Some(x) => {
                let word = x.matches(char::is_alphanumeric).collect::<Vec<&str>>().join("").to_lowercase();
                if !word.is_empty() {
                    let counter = map.entry(word).or_insert(0);
                    *counter += 1;
                }
            },
            None => break,
        }
    }
    map
}
