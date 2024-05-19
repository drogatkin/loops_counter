//use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::File;
use std::io::Read;
use std::convert::TryInto;


fn main() {
   let mut exa = ran_str(25);
   println!{ "str - {exa}" };
   exa = ran_str(25);
   println!{ "str - {exa}" };
   exa = ran_str(5);
   println!{ "str - {exa}" };
   exa = ran_str(14);
   println!{ "str - {exa}" };
   exa = ran_str(235);
   println!{ "str - {exa}" };
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
    //3
}
