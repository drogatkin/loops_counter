use std::time::{Duration};
use std::str;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("Hello, time!");

    let semafor = Arc::new(Mutex::new(false));
    let semafor_clone = Arc::clone(&semafor);
   
    let count = Arc::new(Mutex::new(0i32));
    let count_clone = Arc::clone(&count);
    thread::spawn(move || {
        let mut count = count.lock().unwrap();
        loop {
            let mut lock = semafor.try_lock();
            if let Ok(ref mut semafor) = lock {
                if **semafor {
                    break
                }
            }
            *count += 1;
        } 
    });
    thread::sleep(Duration::new(1, 0));
    *semafor_clone.lock().unwrap() = true;
    let count = *count_clone.lock().unwrap();
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