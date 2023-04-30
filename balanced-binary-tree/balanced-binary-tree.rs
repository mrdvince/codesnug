// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::is_balanced_helper(root.as_ref()).is_some()
    }
    
    fn is_balanced_helper(root: Option<&Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if let Some(node) = root {
            let left_height = Self::is_balanced_helper(node.borrow().left.as_ref());
            let right_height = Self::is_balanced_helper(node.borrow().right.as_ref());
            
            match (left_height, right_height) {
                (Some(lh), Some(rh)) => {
                    if (lh - rh).abs() <= 1 {
                        Some(lh.max(rh) + 1)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            Some(0)
        }
    }
}
