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
    let mut changed = false;

    if let Some(r) = root.clone() {
        let min = r.borrow().val;
        traverse(root, min, &mut rtn,&mut changed);
        return if changed { rtn } else { -1 };
    }
    return -1;
}

fn traverse(root: Option<Rc<RefCell<TreeNode>>>, min: i32, rtn: &mut i32,is_changed: &mut bool) {
    if let Some(r) = root {
        let val = r.borrow().val;
        if *rtn >= val && val > min {
            *rtn = val;
            *is_changed = true;
        }
        
        traverse(r.borrow().left.clone(), min, rtn,is_changed);
        traverse(r.borrow().right.clone(), min, rtn,is_changed);
    }
    return;
}