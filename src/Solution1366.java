import java.lang.reflect.Array;
import java.util.*;

public class Solution1366 {

    class CharAndFrequency implements Comparable<CharAndFrequency> {
        char c;
        int[] votes;
        public CharAndFrequency(char c, int len) {
            this.c = c;
            votes = new int[len];
        }
        @Override
        public int compareTo(CharAndFrequency o) {
            for(int i = 0; i < this.votes.length; i++) {
                if (o.votes[i] - this.votes[i] == 0) {
                    continue;
                }
                return o.votes[i] - this.votes[i];
            }
            return this.c - o.c;
        }
    }

    public String rankTeams(String[] votes) {
        int len = votes[0].length();
        Map<Character, CharAndFrequency> m = new HashMap<>();
        List<CharAndFrequency> l = new ArrayList<>();
        for(String vote: votes) {
            for(int i = 0; i < len; i++) {
                char c = vote.charAt(i);
                if (!m.containsKey(c)) {
                    CharAndFrequency cf = new CharAndFrequency(c, len);
                    m.put(c, cf);
                    l.add(cf);
                }
                m.get(c).votes[i]++;
            }
        }
        StringBuilder sb = new StringBuilder();
        Collections.sort(l);
        for(CharAndFrequency cf: l) {
            sb.append(cf.c);
        }
        return sb.toString();
    }

    public static void main(String[] args) {
        Solution1366 s = new Solution1366();

        System.out.println(s.rankTeams(new String[]{"ABC","ACB","ABC","ACB","ACB"}));
    }
}
