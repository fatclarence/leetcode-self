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
    pub fn reverse_util(prev: Option<Box<ListNode>>, curr: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if curr.is_none() {
            prev // you don't need to write return, but can only do this when you do an if-else
        } else {
            // Get the Box<ListNode> out of the Option
            // Need to make curr_node mutable because when we call take()
            // , it mutates curr_node.next by setting it to None
            let mut curr_node = curr.unwrap();
            

            // take ownership of the next value, replace curr_node.next with None
            let next = curr_node.next.take();
            curr_node.next = prev; // curr.next now owns prev's data

            // must add Some(..) to create a new Option type value containing some data
            // remember the Self::...
            Self::reverse_util(Some(curr_node), next)
        }
    }

    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let prev = None;

        Self::reverse_util(prev, head)
    }
}
