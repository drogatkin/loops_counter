use std::time::{Duration};
use std::str;
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    println!("Hello, time!");
    let atomic = AtomicUsize::new(1);

    let _thread = thread::spawn(move|| {
        thread::sleep(Duration::new(1, 0));
        &atomic.store(0, Ordering::Relaxed);
    });
    let mut count = 0i32;
    // Wait for the other thread to release the lock
    while atomic.load(Ordering::Relaxed) == 1 {
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