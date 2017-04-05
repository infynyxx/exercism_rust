pub fn hamming_distance(str1: &str, str2: &str) -> Result<i32, i32> {
    if str1.len() != str2.len() {
        return Err(0);
    }
    if str1.is_empty() && str2.is_empty() {
        return Ok(0);
    }
    let vec1: Vec<char> = str1.chars().collect();
    let vec2: Vec<char> = str2.chars().collect();
    let length = vec1.len();
    let mut distance = 0;
    for i in 0..length {
        if vec1[i] != vec2[i] {
            distance = distance + 1;
        }
    }
    Ok(distance)
}
