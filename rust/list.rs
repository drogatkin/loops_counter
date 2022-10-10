use std::cmp::Ordering;
use std::io::{self, BufRead};
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node<T> {
  pub val: T,
  pub next: Option<Box<Node<T>>>
}

impl Node<String> {
  #[inline]
  fn new(val: String) -> Self {
    Node {
      next: None,
      val
    }
  }
}

pub fn merge_two_lists(l1: Option<Box<Node<String>>>, l2: Option<Box<Node<String>>>) -> Option<Box<Node<String>>> {
  match  (l1, l2) {
    (Some(l), None) => return Some(l),
    (None, Some(r)) => return Some(r),
    (None, None) => return None,
    (Some(l), Some(r)) => {
      if l.val.cmp(&r.val) == Ordering::Less {
        return Some(Box::new(Node{ next: merge_two_lists(l.next, Some(r)), val: l.val }));
      } else {
        return Some(Box::new(Node{ next: merge_two_lists(Some(l), r.next), val: r.val }));
      }
    },
  }
}

fn print_list(l: Option<Box<Node<String>>>) {
    match l {
        None => println!("."),
        Some(l) => {println!("{0}", l.val); print_list(l.next)},
    }
}

fn find_and_print(s:String, l: Option<Box<Node<String>>>) {
   match l {
       Some(v) => {if v.val.contains(&s) {println!("{}", v.val)} else {println!("no {} in {}", s, v.val) }; find_and_print(s, v.next)},
       None => {},
   }
}

fn main() {
    let val1 = merge_two_lists(Some(Box::new(Node::new("value".to_string()))), Some(Box::new(Node::new("image".to_string()))));
    
     let val2 = Some(Box::new(Node::new("Aa".to_string())));
     
     let mut val : Option<Box<Node<String>>>;

    let res = merge_two_lists(val1, val2);
   // print_list(res);
    val = res;
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");

        match input.trim() {
            "q" => break, //std::process::exit(0),
            _ => {val = merge_two_lists(val, Some(Box::new(Node::new(input.trim().to_string()))));},
        }
    }
    println!("What to find?");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    find_and_print(input.trim().to_string(), val);
    //print_list(val);
   println!("end")
}