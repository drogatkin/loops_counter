use std::time::{Duration};
use std::str;
use std::sync::{Arc};
use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};

fn main() {
    println!("Hello, time!");
    let spinlock = Arc::new(AtomicUsize::new(1));

    let spinlock_clone = Arc::clone(&spinlock);

    let _thread = thread::spawn(move|| {
        thread::sleep(Duration::new(1, 0));
        spinlock_clone.store(0, Ordering::Release);
    });
    let mut count = 0i32;
    // Wait for the other thread to release the lock
    while spinlock.load(Ordering::Acquire) != 0 {
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