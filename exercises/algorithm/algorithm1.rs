/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug, Clone)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T>
where
    T: Clone + Ord, // 确保 T 类型遵循 Clone 和 Ord
{
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T: Clone + Ord> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Ord> LinkedList<T> {
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
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn merge(mut list_a: Self, mut list_b: Self) -> Self {
        let mut merged_list = LinkedList::new();

        let mut cur_a = list_a.start;
        let mut cur_b = list_b.start;

        while let (Some(a), Some(b)) = (cur_a, cur_b) {
            unsafe {
                if (*a.as_ptr()).val <= (*b.as_ptr()).val {
                    merged_list.add((*a.as_ptr()).val.clone());
                    cur_a = (*a.as_ptr()).next;
                } else {
                    merged_list.add((*b.as_ptr()).val.clone());
                    cur_b = (*b.as_ptr()).next;
                }
            }
        }

        // Append the rest of the remaining list
        let mut remaining = if cur_a.is_some() { cur_a } else { cur_b };
        while let Some(node) = remaining {
            unsafe {
                merged_list.add((*node.as_ptr()).val.clone());
                remaining = (*node.as_ptr()).next;
            }
        }

        merged_list
    }
}

impl<T: Display + Clone + Ord> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    fn test_merge_linked_list() {
        let mut list_a = LinkedList::new();
        let mut list_b = LinkedList::new();

        list_a.add(1);
        list_a.add(3);
        list_a.add(5);

        list_b.add(2);
        list_b.add(4);
        list_b.add(6);

        let merged_list = LinkedList::merge(list_a, list_b);
        assert_eq!(merged_list.length, 6);
        assert_eq!(format!("{}", merged_list), "1, 2, 3, 4, 5, 6");
    }
}
