//extern crate xmlparser as xml;

pub struct Node<T> {
  pub val: T,
  pub  next: Option<Box<Node<T>>>
}

impl Node<String> {
  #[inline]
  fn new(val: String, next: Option<Box<Node<String>>>) -> Self {
    Node {
      next,
      val
    }
  }
}


pub struct NodeIter<'li, DATA>(&'li Option<Box<Node<DATA>>>);

impl <'no, NODE>  Iterator for NodeIter<'no, NODE> {
     type Item = &'no NODE;
     
    fn next(&mut self) -> Option<Self::Item> {
   
    match &self.0 {
            Some(node) => {
                self.0 = &node.next;
                Some(&node.val)
            }
            None => None,
        }
    }
}

fn refval(num: &mut i32) -> i32 {
  *num += 5;
  *num
}

fn refvalun(num: & i32) -> i32 {
  *num + 5
}

struct Reader {
  buf: [char;256],
}

fn main() {
    let stringlist = Some(Box::new(Node::new("value".to_string(), Some(Box::new(Node::new("image".to_string(), Some(Box::new(Node::new("star".to_string(), Some(Box::new(Node::new("end".to_string(), None))))))))))));
    
    for li in NodeIter(&stringlist) {
        println!("{}", li)
    }
    
    let mut tuple_tuple = (1u8, 2u16, 'd');
    let eight = 8;
    tuple_tuple.0 = eight;
  
    match tuple_tuple {
        (3,2,'d') => println!("stupid tuple {:?}", tuple_tuple),
        (8,_,s) => println!("our guy {}", s),
        _ => println!("Something else")
    }
    
   /* for token in xmlparser::Tokenizer::from("<tagname name='value'/>") {
      println!("{:?}", token);
    }*/

    let mut sum = 7;
    let mut z = refval(&mut sum);
    println!("before {} after {}", sum, z);
    z = refvalun(&sum);
    println!("before {} after {}", sum, z);
}
