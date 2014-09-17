use std::mem;

// list struct
pub struct LinkedList<T> {
  head: Option<Box<Node<T>>>
}

// list methods
impl<T> LinkedList<T> {
  fn new() -> LinkedList<T> {
    LinkedList{head: None}
  }

  fn add_to_head(&mut self, elt: T) {
    self.push_front_node(box Node::new(elt))
  }

  fn push_front_node(&mut self,  mut node: Box<Node<T>>) {

    match self.head {
      None => {
        self.head = Some(node);
      }
      Some (ref mut head) => {
        mem::swap(head, &mut node);
        head.next = Some(node);
      }
    }
  }

  fn len(&self) -> uint {
    let mut node = &self.head;
    let mut i = 0;

    loop {
      match *node {
        Some(ref n) => {
          i+=1;
          node=&n.next;
        }
        None => {
          return i
        }
      }
    }
  }
}

// node struct
pub struct Node<T> {
  next: Option<Box<Node<T>>>,
  value: T
}

impl<T> Node<T> {
  fn new(v: T) -> Node<T> {
    Node{value: v, next: None}
  }
}



#[test]
fn adds_to_head () {
  let mut list:LinkedList<int> = LinkedList::new();
  list.add_to_head(1);

  assert_eq!(list.len(), 1);
}
