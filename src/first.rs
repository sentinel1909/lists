// in first.rs

// dependencies
use std::mem;

// struct to represent our linked list
pub struct List {
  head: Link,
}

// a link to the next item in the list, a node is wrapped in a Box which gives it a specific allocation on the heap
enum Link {
  Empty,
  More(Box<Node>)
}

// an individual node, right now can just contain a 32-bit integer
struct Node {
  elem: i32,
  next: Link,
}

// impl block of methods for our linked list
impl List {
  // method to make a new linked list
  pub fn new() -> Self {
    List { head: Link::Empty }
  }

  // method to push new data onto the linked list
  pub fn push(&mut self, elem: i32) {
    let new_node = Box::new(Node {
      elem: elem,
      next: mem::replace(&mut self.head, Link::Empty),
    });

    self.head = Link::More(new_node);
  }

  // method to take data off the linked list
  pub fn pop(&mut self) -> Option<i32> {
    match mem::replace(&mut self.head, Link::Empty) {
      Link::Empty => None,
      Link::More(node) => {
        self.head = node.next;
        Some(node.elem)
      }
    }
  }
}



