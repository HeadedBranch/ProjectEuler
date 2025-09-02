fn main() {
    let mut n = 1;
    'outer: loop {
        for i in (2..=20).rev() {
            if n % i != 0 {
                n += i - (n % i);
                continue 'outer;
            }
        }
        break;
    }
    println!("{n}");
}
