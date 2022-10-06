use std::cmp::Ordering;
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

fn main() {
    let val1 = merge_two_lists(Some(Box::new(Node::new("value".to_string()))), Some(Box::new(Node::new("image".to_string()))));
    
     let val2 = Some(Box::new(Node::new("Aa".to_string())));

    let res = merge_two_lists(val1, val2);
    print_list(res);
}