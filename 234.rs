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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        // Use two pointers (slow and fast) to locate the middle of the list
        let mut slow = &head;
        let mut fast = &head;

        while let Some(fast_node) = fast {
            fast = &fast_node.next;
            if let Some(fast_next) = fast {
                fast = &fast_next.next;
                slow = &slow.as_ref().unwrap().next;
            }
        }

        // Reverse the second half of the list starting from the middle
        let mut prev = None;
        let mut curr = slow.clone();
        while let Some(mut curr_node) = curr {
            curr = curr_node.next.take();
            curr_node.next = prev;
            prev = Some(curr_node);
        }

        // Compare the first half with the reversed second half
        let mut left = &head;
        let mut right = &prev;

        while let Some(right_node) = right {
            if left.as_ref().unwrap().val != right_node.val {
                return false; // Return false if any mismatch is found
            }
            left = &left.as_ref().unwrap().next;
            right = &right_node.next;
        }

        true // Return true if all values match
    }
}
