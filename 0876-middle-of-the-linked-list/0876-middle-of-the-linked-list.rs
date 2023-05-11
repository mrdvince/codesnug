// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        
        let mut fast = head.clone();
        let mut slow = head;

        while let Some(node) = fast {
            fast = node.next.clone();
            if let Some(_) = fast {
                slow = slow.unwrap().next;
                fast = fast.unwrap().next;
            }
        }

        slow
    }
}

