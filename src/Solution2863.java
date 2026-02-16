import java.util.Stack;

public class Solution2863 {
    // didn't come up with mono stack solution

    // mistake during implementation: nums[i] > stack.peek()
    // compare the number with the index stored in stack
    public int maxSubarrayLength(int[] nums) {
        Stack<Integer> stack = new Stack<>();
        for(int i = 0; i < nums.length; i++) {
            if (stack.isEmpty() || nums[i] > nums[stack.peek()]) {
                stack.push(i);
            }
        }

        int ret = 0;
        for(int i = nums.length - 1; i >= 0 && !stack.isEmpty(); i--) {
            if(nums[i] >= nums[stack.peek()]) continue;

            while(!stack.isEmpty() && nums[i] < nums[stack.peek()]) {
                ret = Math.max(ret, i - stack.pop() + 1);
            }
        }
        return ret;
    }

    public static void main(String[] args) {
        Solution2863 solution2863 = new Solution2863();
        int[] array = new int[]{57,55,50,60,61,58,63,59,64,60,63};
        System.out.println(solution2863.maxSubarrayLength(array));
    }
}
