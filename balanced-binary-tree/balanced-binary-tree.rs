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
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        Self::is_balanced_helper(root.as_ref()).is_some()
    }

    fn is_balanced_helper(root: Option<&Rc<RefCell<TreeNode>>>) -> Option<i32> {
        if root.is_none() {
            return Some(0);
        }
        let left_height = Self::is_balanced_helper(root.as_ref().unwrap().borrow().left.as_ref());
        let right_height = Self::is_balanced_helper(root.as_ref().unwrap().borrow().right.as_ref());

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
    }
}
