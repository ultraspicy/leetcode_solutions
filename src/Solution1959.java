public class Solution1959 {
    Integer[][] dp = new Integer[200][200];

    public int minSpaceWastedKResizing(int[] nums, int k) {
        return dp(nums.length - 1, k, nums);
    }

    private int dp(int pos, int k, int[] nums) {
        if (k == -1) return 200 * 1000000;
        if (dp[k][pos] != null || k == pos) return dp[k][pos];

        int ret = 200 * 1000000;
        int sum = 0, max = 0;
        for(int i = pos; i >= k - 1; i--) {
            sum += nums[i];
            max = Math.max(max, nums[i]);
            int wasted = max * (pos - i + 1) - sum;
            ret = Math.min(wasted + dp(i - 1, k - 1, nums), ret);
        }
        dp[k][pos] = ret;
        return ret;
    }


    public static void main(String[] args) {
        Solution1959 s = new Solution1959();
        System.out.println(s.minSpaceWastedKResizing(new int[]{10, 20}, 0));
    }
}
