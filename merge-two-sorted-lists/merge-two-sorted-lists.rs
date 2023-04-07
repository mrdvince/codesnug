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
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;
        let mut l1 = list1;
        let mut l2 = list2;

        while let (Some(node1), Some(node2)) = (l1.as_ref(), l2.as_ref()) {
            if node1.val <= node2.val {
                tail.next = Some(Box::new(ListNode::new(node1.val)));
                l1 = node1.next.clone();
            } else {
                tail.next = Some(Box::new(ListNode::new(node2.val)));
                l2 = node2.next.clone();
            }
            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if l1.is_some() { l1 } else { l2 };
        dummy.next
    }
}
