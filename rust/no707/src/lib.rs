#![feature(ptr_internals)]

use std::convert::TryInto;
use std::marker::PhantomData;
use std::ptr::{self, NonNull};

struct MyLinkedList {
    head: Option<NonNull<Node>>,
    tail: Option<NonNull<Node>>,
    len: usize,
    marker: PhantomData<Box<Node>>,
}

struct Node {
    next: Option<NonNull<Node>>,
    prev: Option<NonNull<Node>>,
    element: i32,
}

impl Node {
    fn new(element: i32) -> Self {
        Node {
            next: None,
            prev: None,
            element,
        }
    }

    pub fn val(&self) -> i32 {
        self.element
    }

    pub fn into_element(self: Box<Self>) -> i32 {
        self.element
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyLinkedList {
    /** Initialize your data structure here. */
    fn new() -> Self {
        MyLinkedList {
            head: None,
            tail: None,
            len: 0,
            marker: PhantomData,
        }
    }

    /** Get the value of the index-th node in the linked list. If the index is invalid, return -1. */
    fn get(&self, index: i32) -> i32 {
        if index >= self.len.try_into().unwrap() {
            return -1;
        }
        unsafe {
            if index == 0 {
                return (*(self.head.unwrap().as_ptr())).val();
            }
            if index == (self.len - 1).try_into().unwrap() {
                return (*(self.tail.unwrap().as_ptr())).val();
            }
            let mut count = 0;
            let mut node: *mut Node = self.head.unwrap().as_ptr();

            loop {
                if count == index {
                    return (*(node)).val();
                }
                node = (*(node)).prev.unwrap().as_ptr();
                count += 1;
            }
        }
    }

    /** Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list. */
    fn add_at_head(&mut self, val: i32) {
        let mut node = Box::new(Node::new(val));
        unsafe {
            node.next = self.head;
            node.prev = None;
           
            let node = Some(Box::into_unique(node).into());

            match self.head {
                None => self.tail = node,
                // Not creating new mutable (unique!) references overlapping `element`.
                Some(head) => (*head.as_ptr()).prev = node,
            }

            self.head = node;
            self.len += 1;
        }
    }

    /** Append a node of value val to the last element of the linked list. */
    fn add_at_tail(&mut self, val: i32) {
        let mut node = Box::new(Node::new(val));
        unsafe {
            node.next = self.tail;
            node.prev = None;
            
            let node = Some(Box::into_unique(node).into());
            match self.tail {
                None => self.head = node,
                Some(tail) => (*tail.as_ptr()).next = node,
            }
            self.tail = node;
            self.len += 1;
        }
    }

    /** Add a node of value val before the index-th node in the linked list. If index equals to the length of linked list, the node will be appended to the end of linked list. If index is greater than the length, the node will not be inserted. */
    fn add_at_index(&self, index: i32, val: i32) {
        if index > (self.len - 1).try_into().unwrap() {
            return;
        }
        
        unsafe {
            let mut count = 0;
            let mut prev: Option<NonNull<Node>> = None;
            let mut node: *mut Node = self.head.unwrap().as_ptr();
            let mut new_node = Box::new(Node::new(val));
            loop {
                if count == index {
                    new_node.prev = prev;
                    new_node.next = NonNull::new(node);
                    
                    let new_node = Some(Box::into_unique(new_node).into());
                    (*(node)).prev = new_node;
                    (*prev.unwrap().as_ptr()).next = new_node;
                    break;
                }
                prev = NonNull::new(node);
                node = (*(node)).prev.unwrap().as_ptr();
                count += 1;
            }
        }
    }

    /** Delete the index-th node in the linked list, if the index is valid. */
    fn delete_at_index(&self, index: i32) {
        if index > (self.len - 1).try_into().unwrap() {
            return;
        }
        unsafe {
            let mut count = 0;
            let mut prev: Option<NonNull<Node>> = None;
            let mut node: *mut Node = self.head.unwrap().as_ptr();

            loop {
                if count == index {
                    (*(*(node)).prev.unwrap().as_ptr()).prev = prev;
                    (*prev.unwrap().as_ptr()).next = (*(node)).prev;
                }
                prev = NonNull::new(node);
                node = (*(node)).prev.unwrap().as_ptr();
                count += 1;
            }
        }
    }
}

/**
 * Your MyLinkedList object will be instantiated and called as such:
 * let obj = MyLinkedList::new();
 * let ret_1: i32 = obj.get(index);
 * obj.add_at_head(val);
 * obj.add_at_tail(val);
 * obj.add_at_index(index, val);
 * obj.delete_at_index(index);
 */

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
