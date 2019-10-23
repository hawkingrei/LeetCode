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

pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    if l1.is_none() {
        return l2;
    }
    if l2.is_none() {
        return l1;
    }
    let mut dummy_head = Some(Box::new(ListNode { val: 0, next: None }));
    let mut head = &mut dummy_head;
    let (mut l1, mut l2) = (l1, l2);
    while l1.is_some() || l2.is_some() {
        if l1.is_none() {
            head.as_mut().unwrap().next = l2;
            break;
        } else if l2.is_none() {
            head.as_mut().unwrap().next = l1;
            break;
        }
        let next = if l1.as_ref().unwrap().val > l1.as_ref().unwrap().val {
            let (origin, next) = take_head(l1);
            l1 = origin;
            next
        } else {
            let (origin, next) = take_head(l2);
            l2 = origin;
            next
        };
        head.as_mut().unwrap().next = next;
        head = &mut head.as_mut().unwrap().next;
    }
    dummy_head.unwrap().next
}

#[inline(always)]
fn take_head(mut l: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
    let l_next = l.as_mut().unwrap().next.take();
    let next = l.take();
    l = l_next;
    (l, next)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
