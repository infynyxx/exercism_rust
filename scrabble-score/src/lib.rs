use std::collections::HashMap;

pub fn score(letter: &str) -> i32 {
    if letter.is_empty() {
        return 0;
    }
    let mut character_map: HashMap<char, i32> = HashMap::with_capacity(26);
    /**
    Letter                           Value
    A, E, I, O, U, L, N, R, S, T       1
    D, G                               2
    B, C, M, P                         3
    F, H, V, W, Y                      4
    K                                  5
    J, X                               8
    Q, Z                               10
    **/
    character_map.insert('A', 1);
    character_map.insert('E', 1);
    character_map.insert('I', 1);
    character_map.insert('O', 1);
    character_map.insert('U', 1);
    character_map.insert('L', 1);
    character_map.insert('N', 1);
    character_map.insert('R', 1);
    character_map.insert('S', 1);
    character_map.insert('T', 1);

    character_map.insert('D', 2);
    character_map.insert('G', 2);

    character_map.insert('B', 3);
    character_map.insert('C', 3);
    character_map.insert('M', 3);
    character_map.insert('P', 3);

    character_map.insert('F', 4);
    character_map.insert('H', 4);
    character_map.insert('V', 4);
    character_map.insert('W', 4);
    character_map.insert('Y', 4);

    character_map.insert('K', 5);

    character_map.insert('J', 8);
    character_map.insert('X', 8);

    character_map.insert('Q', 10);
    character_map.insert('Z', 10);

    let zero_score: i32 = 0;

    letter
        .to_uppercase()
        .chars()
        .into_iter()
        .map(|c| character_map.get(&c).unwrap_or(&zero_score))
        .sum()
}
