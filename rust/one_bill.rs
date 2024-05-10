use std::time::{SystemTime, SystemTimeError};
//use std::io;

fn main() -> Result<(), SystemTimeError> {
    let lim = 10_000_000_000u64;
    let mut sum = 0u64;
    let sys_time = SystemTime::now();
    for _ in 0..lim {
        sum += 1
    }
    println!{"Ten billion {sum} in {}ns", sys_time.elapsed()?.as_nanos()};
    Ok(())
}