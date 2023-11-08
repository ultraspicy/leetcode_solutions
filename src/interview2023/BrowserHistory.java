package interview2023;

import java.util.Stack;

class BrowserHistory {

    Stack<String> back;
    Stack<String> forward;
    String cur;
    public BrowserHistory(String homepage) {
        // setup stack
        back = new Stack<>();
        forward = new Stack<>();
        //back.push(homepage);
        cur = homepage;
    }

    public void visit(String url) {
        // popAll() from the current
        // add url onto top of stack
        back.push(cur);
        cur = url;
        forward.clear();
    }

    public String back(int steps) {
        int tmp = steps;
        while (!back.isEmpty() && tmp-- > 0) {
            forward.push(cur);
            cur = back.pop();
        }
        return cur;
    }

    public String forward(int steps) {
        int tmp = steps;
        while(!forward.isEmpty() && tmp-- > 0) {
            back.push(cur);
            cur = forward.pop();
        }
        return cur;
    }
}

/**
 * Your BrowserHistory object will be instantiated and called as such:
 * BrowserHistory obj = new BrowserHistory(homepage);
 * obj.visit(url);
 * String param_2 = obj.back(steps);
 * String param_3 = obj.forward(steps);
 */