use std::fs;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::io::{self, Write, prelude::*, SeekFrom};
use std::fs::File;
use std::str;

pub enum Format {
    Hex,
    Dec,
    Oct,
}

pub enum Display {
    B16,
    B32,
    B64,
    L16,
    L32,
    L64,
    A,
}

pub enum Cmd {
    Next,
    Quit,
    Pos {
       offset: u64 ,
    },
    Format {
       format : Format,
    },
    Display (Display),
}

fn cmdProc() -> Cmd {
    io::stdout().flush().unwrap();
    print!("\nEnter a command <ENTER> - next, Onnnn - move to the offset, F[D|H|O] - display format, N[1|3|6][l|b], q - exit :  ");
    io::stdout().flush().unwrap();
    let mut line = String::new();
     std::io::stdin().read_line(&mut line).unwrap();
     match line.as_bytes()[0] as char {
         '\n' => Cmd::Next,
         'f' | 'F' => {
            match line.as_bytes()[1] as char {
                'D' | 'd' => Cmd::Format{ format:Format::Dec},
                'O' | 'o' => Cmd::Format{ format:Format::Oct},
                 _ => Cmd::Format{ format:Format::Hex},
            } 
         },
         'N' | 'n' => {
            let mut big = true;
            if line.len() > 2 {
                if 'l' == line.as_bytes()[2] as char { big = false; }
            }
            match line.as_bytes()[1] as char {
                '3'  => {
                     if big { Cmd::Display(Display::B32) } else { Cmd::Display(Display::L32) }
                    },
               '6'  => {
                     if big { Cmd::Display(Display::B64) } else { Cmd::Display(Display::L64) }
                    },
                _  => {
                     if big { Cmd::Display(Display::B16) } else { Cmd::Display(Display::L16) }
                    },
            } 
         },
         'O' | 'o' => {
            let n : u64 = line[1..].trim().parse().expect("Number");
            Cmd::Pos{offset : n}
         },
         'q' | 'Q' => Cmd::Quit,
         _ => Cmd::Format{ format:Format::Dec},
     }
}

fn main() -> io::Result<()> {
    println!("hv (HexView)  (c) Copyright {}", 2022);
    const PAGESIZE : u32 = 100;
    
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
           let mut counter : u64 = 0;
           let mut byteCnt = 0;
           let mut pageCnt = 0;
          
           let mut format1 = Format::Dec;
           let mut format2 = Display::A;
           loop {
               if remain > 0 {
                   // print remaining
                   for byte in buf {  // buf[offset..offset+remain]
                        
                        if byteCnt == 0 {
                            print!("\n{:0>8}: ", counter);
                            pageCnt += 1;
                        }
                       if offset == 0 {
                            print!("{:02X} ", byte);
                            match byte {
                                0x0a | 0x0d | 0x1b | 0x07 => strbuf[byteCnt] = 0x2e,
                                _ => strbuf[byteCnt] = byte,
                            }
                            //strbuf[byteCnt] = byte;
                            remain -= 1;
                            byteCnt += 1;
                            if byteCnt == 16 {
                                byteCnt = 0;
                              //  let s = str::from_utf8(&strbuf).unwrap().to_string();
                                 let s = String::from_utf8_lossy(&strbuf);
                                //let s = String::from_utf8(strbuf.to_vec()).expect("Found invalid UTF-8");
                                print!(" {}", s);
                                if pageCnt == PAGESIZE {
                                   pageCnt = 0;
                                   let cmd = cmdProc();
                                   match cmd {
                                       Cmd::Next => (),
                                       Cmd::Quit => return Ok(()),
                                       Cmd::Pos{offset} => { 
                                          counter = offset;
                                          counter -= 1;
                                          f.seek(SeekFrom::Start(counter))?;
                                          remain = 0;
                                          byteCnt = 0;
                                        },
                                       Cmd::Format{format} => format1 = format,
                                       Cmd::Display(d) => format2 = d,
                                   }
                                }
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