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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let node_val = node.borrow().val;
            let p_val = p.as_ref().unwrap().borrow().val;
            let q_val = q.as_ref().unwrap().borrow().val;

            if p_val < node_val && q_val < node_val {
                Self::lowest_common_ancestor(node.borrow().left.clone(), p, q)
            } else if p_val > node_val && q_val > node_val {
                Self::lowest_common_ancestor(node.borrow().right.clone(), p, q)
            } else {
                Some(node)
            }
        } else {
            None
        }
    }
}
