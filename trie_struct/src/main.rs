// use std::collections::HashMap;

// struct TrieNode {
//     is_word: bool,
//     children: HashMap<char, TrieNode>
// }

// impl TrieNode {
//     fn new() -> Self {
//         TrieNode {
//             is_word: false,
//             children: HashMap::new()
//         }
//     }
// }

// struct Trie {
//     root: TrieNode
// }

// /** 
//  * `&self` means the method takes an immutable reference.
//  * If you need a mutable reference, change it to `&mut self` instead.
//  */
// impl Trie {

//     fn new() -> Self {
//         Trie {
//             root: TrieNode::new()
//         }
//     }
    
//     fn insert(&mut self, word: String) {
//         let mut curr = &mut self.root;
//         for c in word.chars() {
//             curr = curr.children
//                     .entry(c) // gets or insert with this key
//                     .or_insert(TrieNode::new());
//             // if !curr.children.contains_key(&c) {
//             //     curr.children.insert(c, TrieNode::new());
//             // }

//             // // Get mutable reference to the child node
//             // curr = curr.children.get_mut(&c).unwrap();
//         }

//         curr.is_word = true;
//     }
    
//     fn search(&self, word: String) -> bool {  
//         let mut curr = &self.root;  // we don't mutate the values in root, so we just need a ref, don't need to own a value at all

//         for c in word.chars() {
//             if !curr.children.contains_key(&c) {
//                 return false;
//             }

//             curr = curr.children.get(&c).unwrap();  // Changed get_mut to get
//         }

//         curr.is_word
//     }
    
//     fn starts_with(&self, prefix: String) -> bool {
//         let mut curr = &self.root;

//         for c in prefix.chars() {
//             if !curr.children.contains_key(&c) {
//                 return false;
//             }

//             curr = curr.children.get(&c).unwrap();
//         }

//         true
//     }
// }

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */


 // Alternative implementation:
 use std::collections::HashMap;

struct TrieNode {
    is_word: bool,
    children: HashMap<char, TrieNode>
}

impl TrieNode {
    fn new() -> Self {
        TrieNode {
            is_word: false,
            children: HashMap::new()
        }
    }
}

struct Trie {
    root: TrieNode
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    fn new() -> Self {
        Trie {
            root: TrieNode::new()
        }
    }
    
    fn insert(&mut self, word: String) {
        let mut curr = &mut self.root;
        for c in word.chars() {
            curr = curr.children
                    .entry(c) // gets or insert with this key
                    .or_insert(TrieNode::new());
            // if !curr.children.contains_key(&c) {
            //     curr.children.insert(c, TrieNode::new());
            // }

            // // Get mutable reference to the child node
            // curr = curr.children.get_mut(&c).unwrap();
        }

        curr.is_word = true;
    }
    
    fn search(&self, word: String) -> bool {
        let mut current = &self.root;
        
        for c in word.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return false,
            }
        }
        
        current.is_word
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut current = &self.root;
        
        for c in prefix.chars() {
            match current.children.get(&c) {
                Some(node) => current = node,
                None => return false,
            }
        }
        
        true
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie_operations() {
        let mut trie = Trie::new();
        
        // Test insert and search
        trie.insert("hello".to_string());
        assert!(trie.search("hello".to_string()));
        assert!(!trie.search("hell".to_string()));
        assert!(!trie.search("helloa".to_string()));

        // Test starts_with
        assert!(trie.starts_with("hel".to_string()));
        assert!(!trie.starts_with("abc".to_string()));

        // Test multiple insertions
        trie.insert("hell".to_string());
        assert!(trie.search("hell".to_string()));
        assert!(trie.search("hello".to_string()));
    }
}
