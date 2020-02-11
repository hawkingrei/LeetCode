use std::cell::RefCell;
use std::i32;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut rtn = i32::MAX;
    if let Some(n) = root.clone() {
        let min = n.borrow().val;
        traverse(root, min, &mut rtn);
        return if rtn == i32::MAX { -1 } else { rtn };
    }
    -1
}

fn traverse(root: Option<Rc<RefCell<TreeNode>>>, min: i32, rtn: &mut i32) {
    if let Some(r) = root {
        let val = r.borrow().val;
        if *rtn > val && val > min {
            *rtn = val;
        }

        traverse(r.borrow().left.clone(), min, rtn);
        traverse(r.borrow().right.clone(), min, rtn);
    }
    return;
}
