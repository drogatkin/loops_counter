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



impl <'no, VAL>  Iterator for &'no Node<VAL>  {
     type Item = &'no Node<VAL>;
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

fn unbox<T>(value: Box<T>) -> T {
    *value
}

struct Factorial {
    curr: u64,
    n: u32,
}

impl Iterator for Factorial {
    // We can refer to this type using Self::Item
    type Item = u64;

    // Here, we define the sequence using `.curr` and `.next`.
    // The return type is `Option<T>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    // We use Self::Item in the return type, so we can change
    // the type without having to update the function signatures.
    fn next(&mut self) -> Option<Self::Item> {
        let np1 = self.n+1;

        self.curr *= np1 as u64;
        self.n = np1;

        // Since there's no endpoint to a Factorial sequence, the `Iterator` 
        // will never return `None`, and `Some` is always returned.
        Some(self.curr)
    }
}

// Returns a Fibonacci sequence generator
fn factorial() -> Factorial {
    Factorial { curr: 1, n: 0 }
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
    
    if let  Some(val) = &val {
       // for el in unbox(val) {
         //   
        //}
      //let it = unbox(val);
      let it = val.next();
    }
    
    for i in factorial().skip(4).take(4) {
        println!("> {}", i);
    }
    
   println!("end")
}
