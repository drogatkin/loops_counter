use std::fs;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::io::{self, Write};
use positioned_io::{RandomAccessFile, ReadAt};

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
            let path1 = path.unwrap().path();
            pathsVer.push(path1);
            
           println!("{}: {}", i, pathsVer.last().unwrap().display());
        }
       
        print!("Enter of a number of an entry? ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let num : usize = line.trim().parse()
            .expect("Please enter number");
        
         if num >= 0 && num < pathsVer.len()  {
            println!("Selected file : {:?}", pathsVer[num]);
            let raf = RandomAccessFile::open(pathsVer[num])?;
            let mut buf = [0; 512];
            let bytes_read = raf.read_at(0, &mut buf)?;
        } else {
            println!("Invalid entry - {}", num);
        }
    }
}
