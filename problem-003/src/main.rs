fn main() {
    let mut num: u64 = 600851475143;
    while 2 % num == 0 {
        num /= 2;
    }
    let mut i = 3;
    while i * i <= num {
        while num % i == 0 {
            num /= i;
        }
        i+=2;
    }
    println!("{num}");
}
