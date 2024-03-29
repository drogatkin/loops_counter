use std::fs;
use std::env;
use std::path::Path;
use std::path::PathBuf;
use std::io::{self, Write, prelude::*, SeekFrom};
use std::fs::File;
use std::str;
use std::io::{Error, ErrorKind};

#[derive(PartialEq)]
pub enum Format {
    Hex,
    Dec,
    Oct,
}

pub enum Display {
    S16,
    S32,
    S64,
    A,
}

pub enum Ending {
  BE,
  LE
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
    Display (Display, Ending)
}

const PAGESIZE : u32 = 100;

fn cmdProc() -> Cmd {
    io::stdout().flush().unwrap();
    print!("\n\x1b[1;32m;2mEnter a command <ENTER> - next, Onnnn - move to the offset, F[D|H|O] - display format, N[1|3|6|a][l|b], q - exit :  \x1b[0m");
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
            let mut end = Ending::BE;
            if line.len() > 2 {
                if 'l' == line.as_bytes()[2] as char { end = Ending::LE; }
            }
            match line.as_bytes()[1] as char {
                '3'  => {
                     Cmd::Display(Display::S32, end)
                    },
               '6'  => {
                     Cmd::Display(Display::S64, end)
                    },
                'a' | 'A'  => {
                     Cmd::Display(Display::A, end)
                    },
                _  => {
                     Cmd::Display(Display::S16, end)
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

#[inline]
fn BigEndian_read_u16(buf: &[u8]) -> u16 {
    buf[0] as u16 | (buf[1] as u16)<<8
}

#[inline]
fn LittleEndian_read_u16(buf: &[u8]) -> u16 {
    buf[1] as u16 | (buf[0] as u16)<<8
}

#[inline]
fn BigEndian_read_u32(buf: &[u8]) -> u32 {
    buf[0] as u32 |  (buf[1] as u32)<<8 | (buf[2] as u32)<<16 | (buf[3] as u32)<<24
    //u32::from_be_bytes(buf)
}

#[inline]
fn LittleEndian_read_u32(buf: &[u8]) -> u32 {
    buf[3] as u32 | (buf[2] as u32)<<8 | (buf[1] as u32)<<16 | (buf[0] as u32)<<24
}

#[inline]
fn BigEndian_read_u64(buf: &[u8]) -> u64 {
    buf[0] as u64 | (buf[1] as u64)<<8 | (buf[2] as u64)<<16 | (buf[3] as u64)<<24 | (buf[4] as u64)<<32 | (buf[5] as u64)<<40 | (buf[6] as u64)<<48 | (buf[7] as u64)<<56
}

#[inline]
fn LittleEndian_read_u64(buf: &[u8]) -> u64 {
    buf[7] as u64 | (buf[6] as u64)<<8 | (buf[5] as u64)<<16 | (buf[4] as u64)<<24 | (buf[3] as u64)<<32 | (buf[2] as u64)<<40 | (buf[1] as u64)<<48 | (buf[0] as u64)<<56
}

fn dumpFile(path : &str) -> io::Result<()> {
    let mut buf = [0; 256];
    let mut strbuf = [0; 16];
   // let mut f = File::open(file2.to_owned().into());
   //let mut f = io::Cursor::<Vec<u8>>::new(file2.to_owned().into());
   let mut f = File::open(path)?;
   let mut remain : i64 = 0;
   let mut offset = 0;
   let mut counter : u64 = 0;
   let mut byteCnt = 0;
   let mut pageCnt = 0;
  
   let mut format1 = Format::Dec;
   let mut format2 = Display::A;
   let mut format3 = Ending::BE;
   loop {
       if remain > 0 {
           // print remaining
           for byte in &buf {  // buf[offset..offset+remain]
                
                if byteCnt == 0 {
                    if format1 == Format::Hex {
                        print!("\n{:0>8x}: ", counter);
                    } else if format1 == Format::Oct {
                        print!("\n{:0>12o}: ", counter);
                    } else {
                      print!("\n{:0>8}: ", counter);
                    }
                    pageCnt += 1;
                }
               if offset == 0 {
                    print!("{:02X} ", byte);
                    match byte {
                        0x0a | 0x0d | 0x1b | 0x07 | 0x08 | 0x09 | 0x0c => strbuf[byteCnt] = 0x2e,
                        _ => strbuf[byteCnt] = *byte,
                    }
                    //strbuf[byteCnt] = byte;
                    remain -= 1;
                    byteCnt += 1;
                    if byteCnt == 16 {
                        byteCnt = 0;
                        match format2 {
                           Display::A => {
                               //  let s = str::from_utf8(&strbuf).unwrap().to_string();
                                let s = String::from_utf8_lossy(&strbuf);
                                //let s = String::from_utf8(strbuf.to_vec()).expect("Found invalid UTF-8");
                                 print!(" {}", s);
                           } ,
                           Display::S16 => {
                               for ss in 0..8 {
                                   match format3 {
                                       Ending::BE => {
                                           print!("{:<6} ", BigEndian_read_u16(&strbuf[ss*2..ss*2+2]));
                                       } ,
                                        Ending::LE => {
                                           print!("{:<6} ", LittleEndian_read_u16(&strbuf[ss*2..ss*2+2]));
                                       }
                                  }   
                               }
                           },
                           Display::S32 => {
                               for ss in 0..4 {
                                   match format3 {
                                      Ending::BE => {
                                           print!("{:<10} ", BigEndian_read_u32(&strbuf[ss*4..ss*4+4]));
                                       } ,
                                        Ending::LE => {
                                           print!("{:<10} ", LittleEndian_read_u32(&strbuf[ss*4..ss*4+4]));
                                       }
                                  }   
                               }
                           },
                           Display::S64 => {
                               for ss in 0..2 {
                                   match format3 {
                                       Ending::BE => {
                                           print!("{:<14} ", BigEndian_read_u64(&strbuf[ss*8..ss*8+8]));
                                       } ,
                                        Ending::LE => {
                                           print!("{:<14} ", LittleEndian_read_u64(&strbuf[ss*8..ss*8+8]));
                                       }
                                  }   
                               }
                           },
                        }
                      
                        
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
                               Cmd::Display(d, e) => {format2 = d; format3 = e}
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
    Ok(())
}

fn main() -> io::Result<()> {
     println!("hv (HexView) v 1.0 (c) Copyright {} Unknown", 2022);
    
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
            
            return dumpFile(&file2);
        } else {
            println!("Invalid entry - {}", num);
        }
    } else {
        return dumpFile(&file);
    }
    Err(Error::new(ErrorKind::Other, "Invalid file"))
}