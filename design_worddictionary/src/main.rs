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

struct WordDictionary {
    root: TrieNode
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {

    fn new() -> Self {
        WordDictionary {
            root: TrieNode::new()
        }
    }
    
    fn add_word(&mut self, word: String) {
        let mut curr = &mut self.root;

        for c in word.chars() {
            curr = curr.children.entry(c).or_insert(TrieNode::new());
        }

        curr.is_word = true;
    }

    fn search_util(root_node: &TrieNode, word: &String, index: usize) -> bool {
        if word.len() == index {
            return root_node.is_word;
        }

        // do not take ownership of word
        let curr_char = word.chars().nth(index).unwrap();

        if curr_char == '.' {
            for child in root_node.children.values() {
                if Self::search_util(child, word, index + 1) {
                    return true;
                }
            }
            // character provided is a dot but there are no more values to iterate
            // to find the appropriate subfix
            return false;
        } else {
            // curr character is not a .
            match root_node.children.get(&curr_char) {
                Some(node) => Self::search_util(node, word, index + 1),
                None => false
            }
        }
    }
    
    fn search(&self, word: String) -> bool {
        return Self::search_util(&self.root, &word, 0_usize);
    }
}