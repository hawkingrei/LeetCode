use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
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

pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    balanced_helper(root.as_ref()).is_some()
}

pub fn balanced_helper(root: Option<&Rc<RefCell<TreeNode>>>) -> Option<usize> {
    if let Some(node) = root {
        let pair = (balanced_helper(node.borrow().right.as_ref()), balanced_helper(node.borrow().left.as_ref()));
        match pair {
            (Some(right), Some(left)) => {
                if right > left {
                    if right - left <= 1 {
                        return Some(right + 1);
                    }
                    return None;
                } else {
                    if left - right <= 1 {
                        return Some(left + 1);
                    }
                    return None;
                }
            }
            _ => return None,
        }
    } else {
        return Some(0);
    }
}
