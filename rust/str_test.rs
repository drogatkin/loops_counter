//use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::Read;
use std::convert::TryInto;
use std::time::{SystemTime};


fn main() {
   let sys_time = SystemTime::now();
   
   let ran_vals = [ran_str(6), ran_str(25), ran_str(14), ran_str(235) , ran_str(155)];
   println!{ "str(s) - {ran_vals:?}" };
   let mut res = 0;
   for i in 1..100_000 {
       let str = match i {
       1 => 
           format!("{}", &ran_vals[0]),
       2 => format!("{}{}",&ran_vals[0], &ran_vals[1]),
       3 => format!["{}{}{}",ran_vals[1] , &ran_vals[2] , &ran_vals[0]],
       4 => format!{"{}{}{}{}",ran_vals[3] , &ran_vals[1] , &ran_vals[2] , &ran_vals[0]},
       5 => format!("{}{}{}{}{}",ran_vals[3] , &ran_vals[1] , &ran_vals[2] , &ran_vals[0] , &ran_vals[4]),
       _ => {
           let mut res_str = ran_vals[4].clone();
           for j in 1..i {
               res_str.push_str(&ran_vals[j%ran_vals.len()])
           }
           res_str
       }
       }; 
       if let Some(pos) = str.find("A") {
           res += pos 
       }
       if let Some(pos) = str.rfind("z") {
           res += pos 
       }
   }
   println!{ "Result: {res}" };
   if let Ok(elapsed) = sys_time.elapsed() {
         println!("Finished in {}.{:<03} sec(s)", elapsed.as_secs(), elapsed.subsec_millis())
   }
}

fn ran_str(l:usize) -> String {
    let mut res = String::from("");
    for _ in 1..l {
        let num = gen_rand();
        res.push(char::from_u32('A' as u32 + num ).unwrap());
    }
    res
}

fn gen_rand() -> u32 {
    /*let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos(); */
    let mut f = File::open("/dev/urandom").unwrap();
    let mut buf = [0u8; 16];
    f.read_exact(&mut buf).unwrap();
    let res = u32::from_be_bytes(buf[0..4].try_into().unwrap());
    res % 64
}
