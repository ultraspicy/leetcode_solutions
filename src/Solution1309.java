public class Solution1309 {
    // traverse from right to left, so don't have to check (i + 2)
    public String freqAlphabets(String s) {
        StringBuilder sb = new StringBuilder();
        for(int i = s.length() - 1; i >= 0; i--) {
            char c = s.charAt(i);
            if (c == '#') {
                sb.append((char)((s.charAt(i - 1) - '0') + 10 * (s.charAt(i - 2) - '0') - 10 + 'j'));
                i -= 2;
            } else {
                sb.append((char)(c - '1' + 'a'));
            }
        }
        return sb.reverse().toString();
    }
}
