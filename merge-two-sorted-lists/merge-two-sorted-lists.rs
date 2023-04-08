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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;

        while let (Some(node1), Some(node2)) = (list1.as_ref(), list2.as_ref()) {
            if node1.val <= node2.val {
                tail.next = Some(Box::new(ListNode::new(node1.val)));
                list1 = node1.next.clone();
            } else {
                tail.next = Some(Box::new(ListNode::new(node2.val)));
                list2 = node2.next.clone();
            }
            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if list1.is_some() { list1 } else { list2 };
        dummy.next
    }
}
