import java.util.*;
import java.math.*;

public class Solution {

    int M = 1000000000 + 7;

    public static void main(String[] args) {
        Solution solution = new Solution();
        System.out.println("");
    }

    public int numberOfSubstrings(String s) {
        // navie solution is straightforward
        // finding string that is not one-dominant is much easier
        // use prefix sum to compute how many 0 between i and j
        // there are (1 + len) * len / 2 substring
        // everytime we see a 0, we compare with preceeding 0s
        Map<Integer, Integer> map = new HashMap<>();
        int count = 0;
        int nonDominant = 0;

        for (int i = 0; i < s.length(); i++) {
            if (s.charAt(i) == '0') {
                count++;
                for(int index : map.keySet()) {
                    int substringLen = i - index + 1;
                    int zeros = count - map.get(index) + 1;
                    int ones = substringLen - zeros;
                    if (ones < zeros * zeros) {
                        nonDominant++;
                    }
                    nonDominant++;
                }
                map.put(i, count);
            }
        }

        return 0;
    }
}