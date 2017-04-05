pub fn square_of_sum(number: i32) -> i32 {
    let mut sum = 0;
    for i in 1..number + 1 {
        sum = sum + i
    }
    sum * sum
}

pub fn sum_of_squares(number: i32) -> i32 {
    let mut sum = 0;
    for i in 1..number + 1 {
        sum = sum + i * i;
    }
    sum
}

pub fn difference(number: i32) -> i32 {
    square_of_sum(number) - sum_of_squares(number)
}
