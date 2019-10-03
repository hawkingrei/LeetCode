#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn reverse_between(head: Option<Box<ListNode>>, m: i32, n: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    if m == n {
        return head;
    }
    let mut new_head: Option<Box<ListNode>> = head.clone();
    let mut result: Option<Box<ListNode>> = Some(Box::new(ListNode::new(-1)));
    let mut last = &mut result;
    let mut index = 1;
    let mut node_stack = Vec::new();
    loop {
        if new_head.is_some() {
            if index >= m && index <= n {
                let value = new_head.clone().unwrap().val;
                node_stack.push(value);
                new_head = new_head.unwrap().next.clone();
            } else if index < m || index > n + 1 {
                last.as_mut().unwrap().next =
                    Some(Box::new(ListNode::new(new_head.clone().unwrap().val)));
                last = &mut last.as_mut().unwrap().next;
                new_head = new_head.unwrap().next.clone();
            } else if index == n + 1 {
                while !node_stack.is_empty() {
                    last.as_mut().unwrap().next =
                        Some(Box::new(ListNode::new(node_stack.pop().unwrap())));
                    last = &mut last.as_mut().unwrap().next;
                }
            }
            index += 1;
        } else {
            break;
        }
    }
    while !node_stack.is_empty() {
        last.as_mut().unwrap().next = Some(Box::new(ListNode::new(node_stack.pop().unwrap())));
        last = &mut last.as_mut().unwrap().next;
    }
    return result.unwrap().next;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
