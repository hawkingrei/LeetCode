use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let None = root {
            return vec![];
        }
        let mut res = vec![];
        Solution::reverse(&root, &mut (|x| {res.push(x)}));
        res
    }
    
    pub fn reverse<F: FnMut(i32)>(root: &Option<Rc<RefCell<TreeNode>>>, f :&mut  F){
        if let Some(node) = root {
            Solution::reverse(&node.borrow().left,f);
            f(node.borrow().val);
            Solution::reverse(&node.borrow().right,f);
        }        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
