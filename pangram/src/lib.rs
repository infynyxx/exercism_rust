use std::collections::HashSet;

pub fn is_pangram(str: &str) -> bool {
    if str.is_empty() || str.len() < 26 {
        return false;
    }
    let mut set = HashSet::new();

    for char in str.to_uppercase().chars() {
        if char >= 'A' && char <= 'Z' {
            set.insert(char);
        }
    }
    set.len() == 26
}
