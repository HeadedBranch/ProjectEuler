fn main() {
    let mut largest_seen = 0;
    for a in 1..1000 {
        for b in 1..1000 {
            let string = (a*b).to_string();
            let reversed: String = string.chars().rev().collect();
            if string == reversed && a * b > largest_seen {
                largest_seen = a * b;
            }
        }
    }
    println!("{largest_seen}");
}
