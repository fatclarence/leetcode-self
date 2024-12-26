class BrowserHistory {
  public class Node {
      String url;
      Node next, prev;

      public Node(String url) {
          this.url = url;
          next = null;
          prev = null;
      }
  }

  private Node curr;

  public BrowserHistory(String homepage) {
      curr = new Node(homepage);
  }

  public void visit(String url) {
      Node node = new Node(url);
      
      // set curr to the new visit page
      node.prev = curr;
      curr.next = node;
      curr = node;
  }

  public String back(int steps) {
      while (steps > 0 && curr.prev != null) {
          curr = curr.prev;
          steps--;
      }

      return curr.url;
  }

  public String forward(int steps) {
      while (steps > 0 && curr.next != null) {
          curr = curr.next;
          steps--;
      }

      return curr.url;
  }
}