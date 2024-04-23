/*
	double linked list reverse
	This problem requires you to reverse a doubly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            prev: None,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn reverse(&mut self) {
        let mut current = self.start;
        std::mem::swap(&mut self.start, &mut self.end);
        while let Some(mut node_ptr) = current {
            unsafe {
                let mut node = node_ptr.as_mut();
                std::mem::swap(&mut node.next, &mut node.prev);
                current = node.prev;
            }
        }
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        let mut current = self.start;
        let mut count = 0;
        while let Some(node) = current {
            unsafe {
                if count == index {
                    return Some(&(*node.as_ptr()).val);
                }
                current = (*node.as_ptr()).next;
                count += 1;
            }
        }
        None
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.start;
        let mut first = true;
        while let Some(node) = current {
            unsafe {
                if !first {
                    write!(f, ", ")?;
                }
                write!(f, "{}", (*node.as_ptr()).val)?;
                current = (*node.as_ptr()).next;
                first = false;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_linked_list_1() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![2, 3, 5, 11, 9, 7];
        let reverse_vec = vec![7, 9, 11, 5, 3, 2];
        for i in original_vec {
            list.add(i);
        }
        println!("Original Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for (i, &val) in reverse_vec.iter().enumerate() {
            assert_eq!(Some(&val), list.get(i as i32));
        }
    }

    #[test]
    fn test_reverse_linked_list_2() {
        let mut list = LinkedList::<i32>::new();
        let original_vec = vec![34, 56, 78, 25, 90, 10, 19, 34, 21, 45];
        let reverse_vec = vec![45, 21, 34, 19, 10, 90, 25, 78, 56, 34];
        for i in original_vec {
            list.add(i);
        }
        println!("Original Linked List is {}", list);
        list.reverse();
        println!("Reversed Linked List is {}", list);
        for (i, &val) in reverse_vec.iter().enumerate() {
            assert_eq!(Some(&val), list.get(i as i32));
        }
    }
}
