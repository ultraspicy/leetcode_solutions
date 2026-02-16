import java.util.HashMap;
import java.util.Map;

public class Solution1397 {
    public static void main(String[] args) {
        Solution1397 s = new Solution1397();
        System.out.println(s.findGoodStrings(3, "szc", "zyi", "sdka"));
    }

    private final int M = (int)1e9 + 7;

    public int findGoodStrings(int n, String s1, String s2, String evil) {
        Map<String, Integer> memset = new HashMap<>();
        int[] kmp = KMP(evil);
        return dfs(s1, s2, evil, memset, kmp, 0, 0, true, true);
    }

    private int dfs (String s1, String s2, String evil, Map<String, Integer> memset, int[]kmp, int i, int j, boolean isPrefix, boolean isSuffix) {
        if (j == evil.length()){
            return 0;
        }
        if (i == s1.length()) {
            return 1;
        }
        String key = i + "#" + j + "#" + isPrefix + "#" + isSuffix;
        if (memset.containsKey(key)) return memset.get(key);

        long ret = 0;
        char from = isPrefix ? s1.charAt(i) : 'a', to = isSuffix ? s2.charAt(i) : 'z';
        for (char c = from; c <= to; c++) {
            int nj = j;
            while (nj > 0 && evil.charAt(nj) != c) {
                nj = kmp[nj - 1];
            }
            if (evil.charAt(nj) == c) {
                nj++;
            }
            ret += dfs(s1, s2, evil, memset, kmp, i + 1, nj, isPrefix && c == from, isSuffix && c == to);
            ret = ret % M;
        }
        memset.put(key, (int)ret);
        return (int)ret;
    }

    private int[] KMP (String p) {
        String w = p;
        int boarder = 0, l = w.length();
        int[] s = new int[l];

        for(int i = 1; i < l; i++) {
            while (boarder > 0 && w.charAt(i) != w.charAt(boarder)) {
                boarder = s[boarder - 1];
            }
            if(w.charAt(boarder) == w.charAt(i)) {
                boarder += 1;
            }
            s[i] = boarder;
        }
        return s;
    }
}
