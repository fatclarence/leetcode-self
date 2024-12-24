class SolutionTest {
  public static void main(String[] args) {
      // Create a sample linked list: 1 -> 2 -> 3 -> 4 -> 5
      ListNode head = new ListNode(1);
      head.next = new ListNode(2);
      head.next.next = new ListNode(3);
      head.next.next.next = new ListNode(4);
      head.next.next.next.next = new ListNode(5);

      // Print original list
      System.out.println("Original list:");
      printList(head);

      // Reverse the list
      Solution solution = new Solution();
      ListNode reversed = solution.reverseList(head);

      // Print reversed list
      System.out.println("\nReversed list:");
      printList(reversed);
  }

  // Helper method to print the linked list
  private static void printList(ListNode head) {
      ListNode current = head;
      while (current != null) {
          System.out.print(current.val + " -> ");
          current = current.next;
      }
      System.out.println("null");
  }
}