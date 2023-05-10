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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_diameter = 0;
        Self::max_depth(&root, &mut max_diameter);
        max_diameter
    }

    fn max_depth(root: &Option<Rc<RefCell<TreeNode>>>, diameter: &mut i32) -> i32 {
        if let Some(node) = root {
            let left_depth = Self::max_depth(&node.borrow().left, diameter);
            let right_depth = Self::max_depth(&node.borrow().right, diameter);
            *diameter = (*diameter).max(left_depth + right_depth);
            1 + left_depth.max(right_depth)
        } else {
            0
        }
    }
}
