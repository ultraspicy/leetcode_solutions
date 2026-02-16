public class Solution1381 {
    class CustomStack {

        int[] stack;
        int size;

        public CustomStack(int maxSize) {
            stack = new int[maxSize];
            size = 0;
        }

        public void push(int x) {
            if (size < stack.length) {
                this.stack[size++] = x;
            }
        }

        public int pop() {
            if (size > 0) {
                int ret = this.stack[--size];
                return ret;
            }
            return -1;
        }

        public void increment(int k, int val) {
            for(int i = 0; i < k && i < size; i++) {
                this.stack[i] += val;
            }
        }

//        public static void main(String[] args) {
//            CustomStack customStack = new CustomStack(3);
//            customStack.push(1);                          // stack becomes [1]
//            customStack.push(2);                          // stack becomes [1, 2]
//            customStack.pop();                            // return 2 --> Return top of the stack 2, stack becomes [1]
//            customStack.push(2);                          // stack becomes [1, 2]
//            customStack.push(3);                          // stack becomes [1, 2, 3]
//            customStack.push(4);                          // stack still [1, 2, 3], Don't add another elements as size is 4
//            customStack.increment(5, 100);                // stack becomes [101, 102, 103]
//            customStack.increment(2, 100);                // stack becomes [201, 202, 103]
//            customStack.pop();                            // return 103 --> Return top of the stack 103, stack becomes [201, 202]
//            customStack.pop();                            // return 202 --> Return top of the stack 102, stack becomes [201]
//            customStack.pop();                            // return 201 --> Return top of the stack 101, stack becomes []
//            customStack.pop();
//        }
    }

}