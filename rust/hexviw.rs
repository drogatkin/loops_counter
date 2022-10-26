use std::fs;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::io::{self, Write, prelude::*, SeekFrom};
use std::fs::File;
use std::str;

fn main() -> io::Result<()> {
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
            let mut buf = [0; 256];
            let mut strbuf = [0; 16];
           // let mut f = File::open(file2.to_owned().into());
           //let mut f = io::Cursor::<Vec<u8>>::new(file2.to_owned().into());
           let mut f = File::open(file2)?;
           let mut remain : i64 = 0;
           let mut offset = 0;
           let mut counter : i64 = 0;
           let mut byteCnt = 0;
           loop {
               if remain > 0 {
                   // print remaining
                   for byte in buf {  // buf[offset..offset+remain]
                        if byteCnt == 0 {
                            print!("\n{:0>8}: ", counter);
                        }
                       if offset == 0 {
                            print!("{:02X} ", byte);
                            match byte {
                                0x0a | 0x0d => strbuf[byteCnt] = 0x2e,
                                _ => strbuf[byteCnt] = byte,
                            }
                            //strbuf[byteCnt] = byte;
                            remain -= 1;
                            byteCnt += 1;
                            if byteCnt == 16 {
                                byteCnt = 0;
                                let s = str::from_utf8(&strbuf).unwrap().to_string();
                                //let s = String::from_utf8(strbuf.to_vec()).expect("Found invalid UTF-8");
                                print!(" {}", s);
                            }
                            counter += 1;
                            if remain == 0 {
                                break;
                            }
                        } else {
                            offset -= 1;
                        }
                    }
               }
               let n = f.read(&mut buf[..])?; // .expect("failed to create file").
              // println!("Read - {}", n);
               if n == 0 {
                   //println!("eof");
                   for _ in 0..16-byteCnt {
                       print!("   ");
                   }
                    let s = str::from_utf8(&strbuf[0..byteCnt]).unwrap().to_string();
                    println!(" {}", s);
                   break;
               }
    
               remain = n as i64;
               offset = 0;
               
            }
        } else {
            println!("Invalid entry - {}", num);
        }
    }
    Ok(())
}