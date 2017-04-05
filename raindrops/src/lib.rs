pub fn raindrops(number: i32) -> String {
    let mut string = String::new();
    let factors = generate_factors(number);

    if factors.contains(&3) {
        string.push_str("Pling");
    }
    if factors.contains(&5) {
        string.push_str("Plang");
    }
    if factors.contains(&7) {
        string.push_str("Plong");
    }

    if string.is_empty() {
        format!("{}", number)
    } else {
        string
    }
}

fn generate_factors(number: i32) -> Vec<i32> {
    let mut factors: Vec<i32> = vec![];
    for n in 1..number + 1 {
        if number % n == 0 {
            factors.push(n)
        }
    }
    factors
}
