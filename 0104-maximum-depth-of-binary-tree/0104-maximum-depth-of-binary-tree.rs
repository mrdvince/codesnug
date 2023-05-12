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
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::depth(&root)
    }
    fn depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut depth = 0;
        if let Some(node) = root {
            let left_depth = Self::depth(&node.borrow().left);
            let right_depth = Self::depth(&node.borrow().right);
            depth = 1 + left_depth.max(right_depth)
        }
        depth
    }
}
