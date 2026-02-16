package minmaxstack;

import java.util.Stack;

public class MinStack {
    Stack<Integer>  stack;
    Stack<Integer> minStack;
    public MinStack() {
        stack = new Stack<>();
        minStack = new Stack<>();
    }

    public void push(int number) {
        stack.push(number);
        if(minStack.isEmpty() || minStack.peek() > number) {
            minStack.push(number);
        } else {
            minStack.push(minStack.peek());
        }
    }

    public int pop() {
        minStack.pop();
        return stack.pop();
    }

    public int min() {
        return minStack.peek();
    }
}