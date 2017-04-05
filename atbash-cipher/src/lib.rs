static ASCII_LOWER: [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i',
                                    'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
                                    's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

pub fn encode(s: &str) -> String {
    let lower = s.to_lowercase();
    let mut iter = lower.chars().into_iter();
    let mut vec = Vec::new();
    let mut word_limit_count = 0;
    loop {
        if word_limit_count == 5 {
            vec.push(' ');
            word_limit_count = 0;
        }
        match iter.next() {
            Some(x) => {
                let opt_char = get_corresponding_char(x);
                if opt_char.is_some() {
                    vec.push(opt_char.unwrap());
                    word_limit_count += 1;
                }
            },
            None => break,
        }
    }
    // trim() is called to delete optional whitespace at the end;
    // not the prettiest thing to do but works for now
    vec.into_iter().collect::<String>().trim().to_string()
}

pub fn decode(s: &str) -> String {
    let lower = s.to_lowercase();
    let mut iter = lower.chars().into_iter();
    let mut vec = Vec::new();
    loop {
        match iter.next() {
            Some(x) => {
                let opt_char = get_corresponding_char(x);
                if opt_char.is_some() {
                    vec.push(opt_char.unwrap());
                }
            },
            None => break,
        }
    }
    vec.into_iter().collect::<String>()
}

fn get_corresponding_char(x: char) -> Option<char> {
    if x >= 'a' && x <= 'z' {
        let position = ASCII_LOWER.iter().position(|&c| c == x).unwrap();
        let corresponding_char = if position <= 12 {
            ASCII_LOWER[25 - position]
        } else {
            ASCII_LOWER[25 - position]
        };
        Some(corresponding_char)
    } else if x.is_digit(10) {
        Some(x)
    } else {
        None
    }
}
