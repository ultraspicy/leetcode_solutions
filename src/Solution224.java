import java.util.Stack;

public class Solution224 {
    /**
     * https://leetcode.com/problems/basic-calculator/
     * Use recursive call to unmarshal the parentheses
     * [] - square brackets
     * {} - curly braces
     */
    public int calculate_non_stack(String s) {
        int res = 0, idx = 0, sign = 1;

        while(idx < s.length()) {
            char c = s.charAt(idx);

            if (c >= '0' && c <= '9') {
                int operand = 0;
                while (idx < s.length() && s.charAt(idx) >= '0' && s.charAt(idx) <= '9') {
                    operand = operand * 10 + s.charAt(idx) - '0';
                    idx++;
                }
                res += sign * operand;
            }  else if (c == '-') {
                sign = -1;
                idx++;
            } else if (c == '+') {
                sign = 1;
                idx++;
            } else if (c == '(') {
                int counter = 1, i = idx;
                while (i++ < s.length()) {
                    if(s.charAt(i) == '(') {
                        counter++;
                    }
                    if (s.charAt(i) == ')') {
                        counter--;
                    }
                    if (counter == 0) break;
                }
                res += sign * calculate_non_stack(s.substring(idx + 1, i));
                idx = i;
            } else {
                idx++;
            }
        }
        return res;
    }

    /**
     * https://leetcode.com/problems/basic-calculator/
     * follow-up use stack but do not store number in stack
     */
    public int calculate(String s) {
        int res = 0;
        Stack<Integer> stack = new Stack<>();
        stack.push(1);
        int sign = 1;

        for(int i = 0; i < s.length(); i++) {
            if (s.charAt(i) >= '0' && s.charAt(i) <= '9') {
                int op = 0;
                while(i < s.length() && s.charAt(i) >= '0' && s.charAt(i) <= '9') {
                    op = op * 10 + s.charAt(i) - '0';
                    i++;
                }
                res += op * sign;
                i--;
            } else if (s.charAt(i) == '(') {
                stack.push(sign);
            } else if (s.charAt(i) == ')') {
                stack.pop();
            } else if (s.charAt(i) == '+') {
                sign = stack.peek();
            } else if (s.charAt(i) == '-') {
                sign = -1 * stack.peek();
            }
        }
        return res;
    }

    public static void main(String[] args) {
        Solution224 s = new Solution224();

        System.out.println(s.calculate("2147483647"));
    }
}
