public class Solution1392 {
    public String longestPrefix(String s) {
        int len = s.length(), index = 0;
        int[] boarder = new int[len];

        for(int i = 1; i < len; i++) {
            while (s.charAt(i) != s.charAt(index) && index > 0) {
                index = boarder[index- 1];
            }
            if (s.charAt(i) == s.charAt(index)) {
                boarder[i] = index++ + 1;
            }
        }
        return s.substring(0, boarder[len - 1]);
    }
}
