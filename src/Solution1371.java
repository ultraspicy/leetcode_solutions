import java.util.*;

public class Solution1371 {
    public static void main(String[] args) {
        Solution1371 s = new Solution1371();

        System.out.println(4<< -1);
    }

    /**
     * 1. we need to efficiently convert letter combinations of "aeiou" into a key  - use a number as a list of binary indicators
     * 2. XOR is efficient for cancelling effect
     */
    public int findTheLongestSubstring(String s) {
        Map<Integer, Integer> position = new HashMap<>();
        int ret = 0, key = 0;
        position.put(key, -1);
        for (int i = 0; i < s.length(); i++) {
            key ^= 1 << ("aeiou".indexOf(s.charAt(i)) + 1) >> 1;
            position.putIfAbsent(key, i);
            ret = Math.max(ret, i - position.get(key));
        }
        return ret;
    }
}
