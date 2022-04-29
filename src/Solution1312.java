public class Solution1312 {
    public int minInsertions(String s) {
        int[][] dp = new int[s.length()][s.length()];
        for (int i = 0; i < s.length(); i++) for(int j = 0; j < s.length(); j++) {
            if (i < j) {
                dp[i][j] = Integer.MAX_VALUE;
            }
        }

        for(int len = 2; len <= s.length(); len++) {
            for(int i = 0, j = i + len - 1; i < s.length() && j < s.length(); i++, j++) {
                if(s.charAt(i) == s.charAt(j)) {
                    dp[i][j] = dp[i + 1][j - 1];
                }
                dp[i][j] = Math.min(dp[i][j], Math.min(dp[i + 1][j], dp[i][j - 1]) + 1);
            }
        }
        return dp[0][s.length() - 1];
    }
}
