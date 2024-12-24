public class Solution {
  public ListNode reverseUtil(ListNode prev, ListNode curr) {
      if (curr == null) {
          return prev;
      }

      // otherwise we should point current to previous and continue the recursive call
      ListNode next = curr.next;
      curr.next = prev;
      return reverseUtil(curr, next);
  }

  // Point curr node to prev node 
  // to do this probably best to use a utility method
  // if the curr node being passed is null, 
  public ListNode reverseList(ListNode head) {
      if (head == null) {
          return null;
      }
      return reverseUtil(null, head);
  }
}