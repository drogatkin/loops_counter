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

fn main() {
    let val1 = Some(Box::new(Node::new("value".to_string())));
    val1.unwrap().next = Some(Box::new(Node::new("image".to_string())));
}