import java.util.ArrayDeque;
import java.util.Deque;

class Solution {
    public boolean isValid(String s) {
        Deque<Character> stack = new ArrayDeque<>();

        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);

            switch (c) {
                case '(':
                    stack.push(')');
                    break;
                case '[':
                    stack.push(']');
                    break;
                case '{':
                    stack.push('}');
                    break;
                default:
                    // if the stack is empty, it means there is no corresponding opening parenthesis
                    if (stack.isEmpty()) {
                        return false;
                    }

                    char head = stack.pop();
                    if (head != c) {
                        return false;
                    }
                    break;
            }
        }

        // if the stack is empty, it means all the parentheses are valid
        return stack.isEmpty();
    }
}