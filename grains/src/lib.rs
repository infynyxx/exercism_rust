pub fn square(s: u32) -> u64 {
    if s > 64 || s < 1 {
        panic!("Square must be between 1 and 64")
    }
    grains_on_chessboard()[s as usize - 1]
}

pub fn total() -> u64 {
    let mut sum = 0;
    for x in grains_on_chessboard().iter() {
        sum = sum + x
    }
    sum
}

fn grains_on_chessboard() -> [u64; 64] {
    let mut chessboard_grains: [u64; 64] = [1; 64]; // initialized all elements to 1
    // 1, 2, 4, 8, 16
    for i in 1..chessboard_grains.len() {
        chessboard_grains[i] = chessboard_grains[i - 1] * 2
    }
    chessboard_grains
}
