use std::fs;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::io::{self, Write, prelude::*, SeekFrom};
use std::fs::File;

fn main() {
    println!("HexViw  (c) Copyright {}", 2022);
    let star = String::from("*");
    let args: Vec<String> = env::args().collect();
    
    let mut file = if args.len() > 1 {
        &args[1]
    } else {
        &star
    };
    //println!("{}", file);
    if !Path::new(file).exists() {
        let wholeDir = &String::from("./");
        if file.eq(&star) {
            file = wholeDir;
        }
        let paths = fs::read_dir(file).unwrap();
        let mut pathsVer = Vec::<PathBuf>::new();
   
        for (i, path) in paths.enumerate() {
            pathsVer.push(path.unwrap().path());
            
            println!("{}: {}", i, pathsVer.last().unwrap().display());
        }
       
        print!("Enter a number of an entry? ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let num : usize = line.trim().parse()
            .expect("Please enter number");
        
         if num >= 0 && num < pathsVer.len()  {
           // println!("Selected file : {:?}", pathsVer[num]);
            let file2 = pathsVer[num].as_path().display().to_string(); // into_os_string().into_string().unwrap()
            println!("Selected file : {}", file2);
            let mut buf = [0_u8;16];
           // let mut f = File::open(file2.to_owned().into());
           let mut f = io::Cursor::<Vec<u8>>::new(file2.to_owned().into());
            f.read_exact(&mut buf[..16]).expect("read failed");
           // println!("{}", hex::encode(&buf));
            for byte in buf {
              print!("{:02X} ", byte);
            }
        } else {
            println!("Invalid entry - {}", num);
        }
    }
}