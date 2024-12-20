import java.util.Deque;
import java.util.ArrayDeque;

class Solution {
    public int evalRPN(String[] tokens) {
        Deque<Integer> stack = new ArrayDeque<>();

        // if its a number push to stack
        // if it is an operator pop from stack 2 number strings 
        for (int i = 0; i < tokens.length; i++) {
            try {
                int num = Integer.parseInt(tokens[i]);
                stack.push(num);
            } catch (Exception e) {
                int b = stack.pop();
                int a = stack.pop();
                String operator = tokens[i];

                switch (operator) {
                    case "+":
                        stack.push(a + b);
                        break;
                    case "-":
                        stack.push(a - b);
                        break;
                    case "*":
                        stack.push(a * b);
                        break;
                    case "/":
                        stack.push(a / b);
                        break;
                }
            }
        }

        return stack.pop();
    }
}