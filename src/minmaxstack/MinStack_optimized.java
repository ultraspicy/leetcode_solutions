package minmaxstack;

import java.util.Stack;

public class MinStack_optimized {
    Stack<Integer>  stack;
    Stack<Integer> minStack;
    public MinStack_optimized() {
        stack = new Stack<>();
        minStack = new Stack<>();
    }

    public void push(int number) {
        stack.push(number);
        if(minStack.isEmpty() || minStack.peek() >= number) {
            minStack.push(number);
        }
    }

    public int pop() {
        int number = stack.pop();
        if(minStack.peek() == number) {
            minStack.pop();
        }
        return number;
    }

    public int min() {
        return minStack.peek();
    }
}