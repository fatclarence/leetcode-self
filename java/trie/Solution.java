import java.util.HashMap;

class Trie {
  private TrieNode root;

  class TrieNode {
      // field initialization happens here
      HashMap<Character, TrieNode> children = new HashMap<>();
      boolean isWord = false;
  }

  public Trie() {
      this.root = new TrieNode();
  }
  
  public void insert(String word) {
      TrieNode curr = this.root;

      for (char c : word.toCharArray()) {
          if (!curr.children.containsKey(c)) {
              // add the new letter to this root
              curr.children.put(c, new TrieNode());
          }

          curr = curr.children.get(c);
      }
      
      curr.isWord = true;
  }
  
  public boolean search(String word) {
      TrieNode curr = this.root;
      for (char c : word.toCharArray()) {
          if (!curr.children.containsKey(c)) {
              // does not contain means the word does not exist
              return false;
          }

          // continue to traverse
          curr = curr.children.get(c);
      }

      return curr.isWord;
  }
  
  public boolean startsWith(String prefix) {
      TrieNode curr = this.root;
      for (char c : prefix.toCharArray()) {
          if (!curr.children.containsKey(c)) {
              return false;
          }
          // continue to traverse
          curr = curr.children.get(c);
      }
      return true;
  }
}

/**
* Your Trie object will be instantiated and called as such:
* Trie obj = new Trie();
* obj.insert(word);
* boolean param_2 = obj.search(word);
* boolean param_3 = obj.startsWith(prefix);
*/