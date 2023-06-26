pub struct LinkedList<'a, T> {
    pub val: Option<T>,
    pub next: Option<&'a Box<LinkedList<'a, T>>>,
}

impl<'a> LinkedList<'a, i32> {
    pub fn new() -> LinkedList<'a, i32>{
        LinkedList {
            val: None,
            next: None,
        }
    }

  pub fn push_left(&mut self, node: &'a mut Box<LinkedList<'a, i32>>) {
      if self.next.is_none() {
          self.next = Some(node);
      } else {
          node.next = self.next;            
          self.next = Some(node);
      }
  }
  
  pub fn create_box(&self, x: i32) -> Box<LinkedList<'a, i32>> {
      Box::new(
          LinkedList {
              val: Some(x),
              next: None
          }
      )
  }
}