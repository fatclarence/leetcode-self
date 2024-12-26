import java.util.Stack;

class BrowserHistory {
  private Stack<String> forwardHistory = new Stack<>();
  private Stack<String> backHistory = new Stack<>();
  private String currPage;

  public BrowserHistory(String homepage) {
      currPage = homepage;
  }
  
  public void visit(String url) {
      // clears up all forward history
      forwardHistory.clear();
      // back history should include the previous page (which is the current page)
      backHistory.push(currPage);
      // visits current page
      currPage = url;
  }
  
  public String back(int steps) {
      if (steps == 0 || backHistory.isEmpty()) {
          return currPage;
      }

      int remaining = steps;

      while (remaining > 0 && !backHistory.isEmpty()) {
          // move current page to forward
          forwardHistory.push(currPage);

          // set currPage
          currPage = backHistory.pop();

          // decrement the remaining pages to move back
          remaining--;
      }

      return currPage;
  }
  
  public String forward(int steps) {
      if (steps == 0 || forwardHistory.isEmpty()) {
          return currPage;
      }

      int remaining = steps;

      while (remaining > 0 && !forwardHistory.isEmpty()) {
          // move current page to forward
          backHistory.push(currPage);

          // set currPage
          currPage = forwardHistory.pop();

          // decrement the remaining pages to move back
          remaining--;
      }

      return currPage;
  }
}

/**
* Your BrowserHistory object will be instantiated and called as such:
* BrowserHistory obj = new BrowserHistory(homepage);
* obj.visit(url);
* String param_2 = obj.back(steps);
* String param_3 = obj.forward(steps);
*/