import java.util.*;

public class Solution3234 {
    public static void main(String[] args) {
        Solution3234 s = new Solution3234();
        System.out.println(s.numberOfSubstrings("00011"));
    }
    public int numberOfSubstrings(String s) {
        // navie solution is straightforward
        // finding string that is not one-dominant is much easier
        // use prefix sum to compute how many 0 between i and j
        // there are (1 + len) * len / 2 substring

        // everytime we see a 0, we compare with preceeding 0s (this is wrong)
        // correct logic is to identify how to skip
        
        int n = s.length();
        int[] prefixOnes = new int[n + 1];
        
        // Calculate prefix sum of ones
        for (int i = 0; i < n; i++) {
            prefixOnes[i + 1] = prefixOnes[i] + (s.charAt(i) - '0');
        }
        
        int ret = 0;
        for (int i = 0; i < n; i++) {
            int j = i;
            while (j < n) {
                int ones = prefixOnes[j + 1] - prefixOnes[i];
                int len = j - i + 1;
                int zeros = len - ones;
                
                if (zeros * zeros > ones) {
                    j += zeros * zeros - ones;
                } else if (zeros * zeros == ones) {
                    ret++;
                    j++;
                } else {
                    int dominantCount = (int)Math.min(Math.sqrt(ones) - zeros, n - j - 1) + 1;
                    ret += dominantCount;
                    j += dominantCount;
                }
            }
        }
        return ret;
    }
}