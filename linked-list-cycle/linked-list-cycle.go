/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
//  No rust runtime for this solution so using GO

func hasCycle(head *ListNode) bool {
    fast := head
    for fast != nil && fast.Next != nil {
        fast = fast.Next.Next
        head = head.Next
        if fast == head {
            // they meet
            return true
        }
    }
    return false
}