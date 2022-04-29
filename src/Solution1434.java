import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Solution1434 {
    public int numberWays(List<List<Integer>> hats) {
        Map<Integer, List<Integer>> h2p = new HashMap<>();
        int n = hats.size();
        for(int i = 0; i < n; i++) {
            List<Integer> h = hats.get(i);
            for(int prefered : h) {
                if (!h2p.containsKey(prefered)) {
                    h2p.put(prefered, new ArrayList<>());
                }
                h2p.get(prefered).add(i);
            }
        }

        Integer[][] dp = new Integer[41][1 << n];
        return dfs(dp, h2p, 1, n, 0, (1 << n) - 1);
    }

    /**
     *
     * @param dp
     * @param h2p the map from hat to it's available people
     * @param i start with ith hat's assignment
     * @param n size of people
     * @param mask state of hats assignment, use int to denote a bit series
     * @return number of ways to assign hats given the constraints
     */
    private int dfs(Integer[][] dp, Map<Integer, List<Integer>> h2p , int i, int n, int mask, int finalMask) {
        if (mask == finalMask) return 1;
        if (i > 40) return 0;

        if (dp[i][mask] != null) return dp[i][mask];
        // we can skip the current hat
        int ret = dfs(dp, h2p, i + 1, n, mask, finalMask);
        if (h2p.containsKey(i)) {
            List<Integer> people = h2p.get(i);
            for(int p : people) {
                if (((mask >> p) & 1) == 1) continue;
                ret = (ret + dfs(dp,h2p, i + 1, n, mask | (1 << p), finalMask)) % 1_000_000_007;
            }
        }
        return dp[i][mask] = ret;
    }
}
