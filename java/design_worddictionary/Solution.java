class WordDictionary {
  static class TrieNode {
      Map<Character, TrieNode> children = new HashMap<>();
      boolean isWord = false;
  }

  private final TrieNode root;

  public WordDictionary() {
      root = new TrieNode();
  }
  
  public void addWord(String word) {
      TrieNode node = root;

      for (char c : word.toCharArray()) {
          // find from hashmap, if not found, add new trie node
          node = node.children.computeIfAbsent(c, k -> new TrieNode());
      }

      node.isWord = true;
  }

  private boolean searchUtil(String word, TrieNode curr) {
      // base case
      if (word.length() == 0) {
          // the fact that this is reached suggests that the word is found
          return curr.isWord;
      }

      char c = word.charAt(0);

      if (c == '.') {
          // .values() returns an empty collection so the for-loop will just not run
          // wont throw NullPointerException
          for (TrieNode child : curr.children.values()) {
              // Must go through every possibility cause if I am finding
              // 'bat' but the first search path goes through 'bad' and returns
              // false then its incorrect
              if (searchUtil(word.substring(1), child)) {
                  return true;
              }
          }
          
          // no child nodes
          return false;
      } else {
          // c is some character possibly found as a child node
          TrieNode child = curr.children.get(c);
          if (child != null) {
              return searchUtil(word.substring(1), child);
          }
          return false;
      }
  }
  
  public boolean search(String word) {
      return searchUtil(word, root);
  }
}

/**
* Your WordDictionary object will be instantiated and called as such:
* WordDictionary obj = new WordDictionary();
* obj.addWord(word);
* boolean param_2 = obj.search(word);
*/