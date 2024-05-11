use std::time::{SystemTime, SystemTimeError};

fn main() -> Result<(), SystemTimeError> {
    let mut num = 20_000;
    let mut sum = 0;
    let sys_time = SystemTime::now();
    let mut j = num;
    while  j >= 1 {
         let mut i = num - 1;
        while i >= 1 {
            if num % i == 0 {
                sum += i;
            }
            i -= 1;
        }
        if num == sum
        {
            println!{"num: {num} / sum:{sum} "};
        }
        sum = 0;
        num -= 1;
        j -= 1;
    }
    let elaps = sys_time.elapsed()?;
    println!{"The program executed in {}.{} sec", elaps.as_secs(), elaps. subsec_millis()};
    Ok(())
}
