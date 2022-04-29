package minmaxstack;

import java.util.Stack;

public class MaxStackNaive {
    Stack<Integer> stack;
    Stack<Integer> maxStack;

    public MaxStackNaive() {
        stack = new Stack<>();
        maxStack = new Stack<>();
    }

    public void push(int x) {
        stack.push(x);
        if(maxStack.isEmpty() || maxStack.peek() <= x) {
            maxStack.push(x);
        }
    }

    public int pop() {
        // write your code here
        int number = stack.pop();
        if(maxStack.peek() == number) {
            maxStack.pop();
        }
        return number;
    }

    public int top() {
        // write your code
        return stack.peek();
    }

    public int peekMax() {
        // write your code here
        return maxStack.peek();
    }

    public int popMax() {
        Stack<Integer> buffer = new Stack<>();
        int ret = maxStack.pop();
        while(stack.peek() != ret) {
            buffer.push(stack.pop());
        }
        stack.pop();
        while (!buffer.isEmpty()) {
            /**
             * IMPORTANT: USE `push()` not `stack.push()`
             */
            push(buffer.pop());
        }
        return ret;
    }
}
