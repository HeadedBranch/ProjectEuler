fn main() {
    let mut sum_of_squares = 0;
    for i in 1..=100 {
        sum_of_squares += i*i;
    }
    let mut sum = 0;
    for i in 1..=100 {
        sum += i;
    }
    println!("{}", sum*sum - sum_of_squares );
}
