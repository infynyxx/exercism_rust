pub fn sum_of_multiples(number: i32, v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in 1..number {
        for element in v {
            if i % element == 0 {
                sum = sum + i;
                break;
            }
        }
    }
    sum
}
