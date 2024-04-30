use std::time::{Duration, SystemTime};
use std::str;
use std::sync::{Arc};
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    println!("Hello, time!");
    let stop = Arc::new(AtomicBool::new(false));

    let stop_clone = stop.clone();
    let now = SystemTime::now();
    let _thread = thread::spawn(move|| {
        thread::sleep(Duration::new(1, 0));
        stop.store(true, Ordering::Relaxed);
    });
    let mut count = 0u64;
    // increment counter while 1 sec passed
    while !stop_clone.load(Ordering::Relaxed)  {
        count += 1;
    }
    let after_1sec = SystemTime::now();
    let num = count
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(","); // separator

    println!("{now:?}\n{after_1sec:?}\ncount: {num}");
}