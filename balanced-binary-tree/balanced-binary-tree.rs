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
        if let Some(node) = root {
            let left_height = Self::height(node.borrow().left.clone());
            let right_height = Self::height(node.borrow().right.clone());
            if (left_height - right_height).abs() > 1 {
                return false;
            }
            let left_side = Self::is_balanced(node.borrow().left.clone());
            let right_side = Self::is_balanced(node.borrow().right.clone());
            return left_side && right_side;
        }
        true
    }
    fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            let left_height = Self::height(node.borrow().left.clone());
            let right_height = Self::height(node.borrow().right.clone());
            return left_height.max(right_height) + 1;
        }
        0
    }
}