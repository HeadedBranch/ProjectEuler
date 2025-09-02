fn main() {
    let mut total = 0;
    let mut cur = 1;
    let mut prev;
    let mut prev_buf = 0;
    while total < 4_000_000 {
        prev = cur;
        cur += prev_buf;
        prev_buf = prev;
        if cur % 2 == 0 { total += cur; }
    }
    println!("{total}");
}
