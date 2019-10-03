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

fn main() {
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut last = &mut head;

    let mut second_node = Some(Box::new(ListNode::new(1)));
    head.as_mut().unwrap().next = second_node.clone();
    head.as_mut().unwrap().val = 100;
    last = &mut head.as_mut().unwrap().next;

    let mut third_node = Some(Box::new(ListNode::new(2)));
    last.as_mut().unwrap().next = third_node;
    last = &mut last.as_mut().unwrap().next;
    println!("{:?}", (&last.as_ref().unwrap()));
}
