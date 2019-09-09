// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

use rand::Rng;
struct Solution {
    head: Option<Box<ListNode>>,
    val : i32,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Solution {

    /** @param head The linked list's head.
        Note that the head is guaranteed to be not null, so it contains at least one node. */
    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut result = Solution{
            head: None,
            val: 0,
        };
        result.head = head.clone();
        if let Some(head_node) = head {
            
            result.val = head_node.val;
        }
        
        result
    }
    
    /** Returns a random node's value. */
    fn get_random(&self) -> i32 {
        let mut result = self.val;
        let mut next = self.head.clone().unwrap().next;
        let mut index = 2;
        loop {
            if let None = next {
                break;
            }
            index += 1;
            let num = rand::thread_rng().gen_range(1, index);
            if num <= 1{
                result = next.clone().unwrap().val;
            }
            next = next.unwrap().next;
        }
        result
    }
}

/**
 * Your Solution object will be instantiated and called as such:
 * let obj = Solution::new(head);
 * let ret_1: i32 = obj.get_random();
 */