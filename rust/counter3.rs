use std::time::{Duration};
use std::str;
use std::sync::{Arc};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    println!("Hello, time!");
    let stop = Arc::new(AtomicBool::new(false));

    let stop_clone = stop.clone();

    let _thread = thread::spawn(move|| {
        thread::sleep(Duration::new(1, 0));
        stop.store(true, Ordering::Relaxed);
    });
    let mut count = 0i32;
    // Wait for the other thread to release the lock
    while !stop_clone.load(Ordering::Relaxed)  {
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