import java.util.Arrays;
import java.util.Map;
import java.util.TreeMap;

public class Solution1370 {
    public static void main(String[] args) {
        Solution1370 s = new Solution1370();

        System.out.println((char)('a' + 3));
    }

    public String sortString(String s) {
        int[] counts = new int[26];
        for (char c : s.toCharArray()) {
            counts[c - 'a']++;
        }
        StringBuilder sb = new StringBuilder();

        int remain = s.length();
        while (remain > 0) {
            for (int i = 0; i < counts.length; i++) {
                if (counts[i] > 0) {
                    sb.append((char)('a' + i));
                    counts[i] -= 1;
                    remain -= 1;
                }
            }
            for (int i = counts.length - 1; i >= 0; i--) {
                if (counts[i] > 0) {
                    sb.append((char)('a' + i));
                    counts[i] -= 1;
                    remain -= 1;
                }
            }
        }
        return sb.toString();
    }
}
