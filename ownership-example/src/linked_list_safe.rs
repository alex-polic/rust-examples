struct LinkedList<'a> {
    value: i32,
    next: Option<&'a Box<LinkedList<'a>>>
}

impl<'a> LinkedList<'a> {
    fn new() -> LinkedList<'a> {
        LinkedList { value: 0, next: None }
    }

    pub fn push_left(&mut self, node: &'a mut Box<LinkedList<'a>>) {
        if self.next.is_none() {
            self.next = Some(node);
        } else {
            node.next = self.next;            
            self.next = Some(node);
        }
    }
    pub fn create_box(&self, x: i32) -> Box<LinkedList<'a>> {
        Box::new(
            LinkedList {
                value: x,
                next: None
            }
        )
    }
  
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list_push_left() {
        let mut list: LinkedList = LinkedList::new();
        let mut node = LinkedList::create_box(&list, 2);
        
        list.push_left(&mut node);
        // list.push_left(2);
        // list.push_left(3);
        // list.push_left(4);
        // assert_eq!(list.collect(), vec![4,3,2,1]);
    }
}