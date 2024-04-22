use std::time::{Duration, Instant};
use std::str;

fn main() {
    println!("Hello, time!");

    let second = Duration::new(1, 0);
    let start = Instant::now();
    let mut count : i32= 0;
    while Instant::now() - start < second {
        count += 1;
    }
    let mut num = count
        .abs()
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(","); // separator
    if count < 0 {
        num = format!("-{num}")
    }
    println!("count: {num}");
}