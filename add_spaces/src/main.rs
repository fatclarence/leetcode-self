impl Solution {
    pub fn add_spaces(s: String, spaces: Vec<i32>) -> String {
        let mut ptr = 0; // ptr is usize
        let spaces_added = spaces.len();
        let mut return_string = String::with_capacity(s.len() + spaces_added);

        for &space_index in &spaces {
            let space_index = space_index as usize;
            // Need to convert i32 to usize for string indexing
        
            // use string splice to push to return_string
            return_string.push_str(&s[ptr..space_index]);
            return_string.push(' ');

            // this is ok because space_index is i32, and thus has copy trait
            ptr = space_index;
        }

        return_string.push_str(&s[ptr..]);
        return_string
    }
}