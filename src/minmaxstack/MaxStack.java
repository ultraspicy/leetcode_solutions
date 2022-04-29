package minmaxstack;

import java.util.HashSet;
import java.util.PriorityQueue;
import java.util.Queue;
import java.util.Stack;

public class MaxStack {

    class Item implements Comparable<Item> {
        int val;
        int index;
        public Item (int val, int index) {
            this.val = val;
            this.index = index;
        }
        @Override
        public int compareTo(Item o) {
            if(o.val != this.val) {
                return o.val - this.val;
            }
            return o.index - this.index;
        }
    }

    private static int globalID;
    private Queue<Item> heap;
    private Stack<Item> stack;
    private HashSet<Item> set;

    public static void main(String[] args) {
        MaxStack maxStack = new MaxStack();
        maxStack.push(5);
        maxStack.push(1);
        maxStack.push(5);
        maxStack.top();
        maxStack.popMax();
        maxStack.top();
        maxStack.peekMax();
        maxStack.pop();
        maxStack.top();
    }
    public MaxStack() {
        heap = new PriorityQueue<>();
        stack = new Stack<>();
        set = new HashSet<>();
        globalID = 0;
    }

    public void push(int x) {
        Item i = new Item(x, globalID++);
        stack.push(i);
        heap.add(i);
    }

    public int pop() {
        while(set.contains(stack.peek())) {
            stack.pop();
        }
        Item ret = stack.pop();
        set.add(ret);
        return ret.val;
    }

    public int top() {
        while(set.contains(stack.peek())) {
            stack.pop();
        }
        return stack.peek().val;
    }

    public int peekMax() {
        while(set.contains(heap.peek())) {
            heap.poll();
        }
        return heap.peek().val;
    }

    public int popMax() {
        while(set.contains(heap.peek())) {
            heap.poll();
        }
        Item ret = heap.poll();
        set.add(ret);
        return ret.val;
    }
}

