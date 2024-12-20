impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' => stack.push(')'), // push the corresponding closing parenthesis
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                _ => {
                    if stack.pop() != Some(c) { // check if the top of the stack is the corresponding closing parenthesis
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}