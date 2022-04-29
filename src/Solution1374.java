public class Solution1374 {
    public static void main(String[] args) {
        Solution1374 s = new Solution1374();

        System.out.println(s.generateTheString(3));
    }

    public String generateTheString(int n) {
        StringBuilder sb = new StringBuilder();

        if (n % 2 == 1) {
            while (n-- > 0) {
                sb.append('a');
            }
        } else {
            while (n-- > 1) {
                sb.append('a');
            }
            sb.append('b');
        }
        return sb.toString();
    }

    /**
     * by lee 215
     * https://leetcode.com/problems/generate-a-string-with-characters-that-have-odd-counts/discuss/532520/JavaC%2B%2BPython-One-Lines
     */
    public String generateTheString_lee(int n) {
        return "b" + "ab".substring(n % 2, 1 + n % 2).repeat(n - 1);
    }
}
