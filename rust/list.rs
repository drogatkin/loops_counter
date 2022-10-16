use std::cmp::Ordering;
//use std::io::{self, BufRead};
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node<T> {
  pub val: T,
  pub next: Option<Box<Node<T>>>
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum List<'LI, DATA> {
    Node {
        val: DATA,
        next: &'LI List<'LI, DATA>,
    },
    Tail
}


impl<DATA> Default for List<'_, DATA> {
    fn default() -> Self {
        List::Tail
    }
}



impl<'LI, DATA> List<'LI, DATA> {
    pub fn add(&'LI self, val: DATA) -> Self {
        List::Node { val, next: self }
    }
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



pub struct ListIter<'li, DATA>(&'li List<'li, DATA>);

impl<'li, NODE> Iterator for ListIter<'li, NODE> {
    type Item = &'li NODE;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.0 {
            List::Node { val, next } => {
                self.0 = next;
                Some(val)
            }
            List::Tail => None,
        }
    }
}



impl<'no> Iterator for &'no Node<String> {
     type Item = &'no Node<String>;
    fn next(&mut self) -> Option<Self::Item> {
        match &self.next {
            None => None,
            Some(e) => {let &mut res=self;   Some(res)},
        }
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
    println!("Enter some list values, 'q' for end");
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
    
    let tail = List::Tail;
    let first = tail.add("apple".to_string());
    let second = first.add("orange".to_string());

    assert_eq!(tail, List::Tail);
    assert_eq!(first, List::Node {
        val: "apple".to_string(),
        next: &List::Tail,
    });
    assert_eq!(second, List::Node {
        val: "orange".to_string(),
        next: &List::Node {
            val: "apple".to_string(),
            next: &List::Tail,
        },
    });
    
   println!("end")
}
