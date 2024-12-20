class Solution {
  public boolean canMakeSubsequence(String str1, String str2) {
      // maintain a pointer that goes through str2
      // go through str1 character by character, comparing
      // to the char and the cyclically incremented char from str1
      // to the curr letter in str2
      
      // alternative is create a hashset
      // add values into hashset from str1 and str1a first.

      // then for each value in str2, add them also
      // if can make subsequence, then each insertion should have yielded a
      // FALSE. (because they should've existed in the Hashset)
      // if cannot make subsequence, one insertion just has to return true
      // HashSet<Character> stringSet = new HashSet<>(16, 0.85f);
      // boolean canMake = true;

      // // O(n)
      // for (int i = 0; i < str1.length(); i++) {
      //     if (str1.charAt(i) == 'z') {
      //         stringSet.add('z');
      //         stringSet.add('a');
      //     } else {
      //         stringSet.add(str1.charAt(i));
      //         stringSet.add((char) (str1.charAt(i) + 1));
      //     }
      // }

      // // O(m)
      // for (int i = 0; i < str2.length(); i++) {
      //     // if insertion returns false, can continue to check
      //     if (stringSet.add(str2.charAt(i))) {
      //         canMake = false;
      //         break;
      //     }
      // }

      // return canMake;
      
      ///////// above doesn't work cause order matters ///////////

      int ptr = 0;

      for (int i = 0; i < str1.length(); i++) {
          
          char fromS1 = str1.charAt(i);
          char fromS1a = fromS1 != 'z' ? (char) (str1.charAt(i) + 1) : 'a';

          if (fromS1 == str2.charAt(ptr) || fromS1a == str2.charAt(ptr)) {
              // move ptr
              ptr++;
          }
          
          if (ptr == str2.length()) {
              return true;
          }
      }

      return false;
  }
}