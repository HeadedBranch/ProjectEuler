fn main() {
    let mut num = 2;
    for _ in 0..=10_001 {
        'outer:loop {
            for i in 2..num/2 {
                if num % i == 0 {
                    num += 1;
                    continue 'outer;
                }
            }
            break;
        }
        num+=1;
    }
    num -= 1;
    println!("{num}");
}
