import java.util.*;

public class StringDeviation {
    public static void main(String[] args) {
        StringDeviation stringDeviation = new StringDeviation();

        String s1 = "baaabbbaaaaaab";
        System.out.println(stringDeviation.stringDeviation(s1));

        String s2 = "abbbcacbcdce";
        System.out.println(stringDeviation.stringDeviation(s2));
    }

    public int stringDeviation(String str) {
        int ret = 0;
        String alphabet = "abcdefghijklmnopqrstuvwxyz";
        for(int i = 0; i < alphabet.length(); i++) {
            for(int j = i + 1; j < alphabet.length(); j++) {
                ret = Math.max(stringDeviationForTwoChars(alphabet.charAt(i), alphabet.charAt(j), str), ret);
            }
        }
        return ret;
    }

    private int stringDeviationForTwoChars(char c1, char c2, String s) {
        List<Integer> l = new ArrayList<>();
        int counter = 0;
        for(char c : s.toCharArray()) {
            if(c == c1) {
                l.add(++counter);
            }
            if(c == c2) {
                l.add(--counter);
            }
        }
        return arrayAmplitude(l);
    }

    /**
     * the amplitude of an array is (max - min)
     */
    private int arrayAmplitude(List<Integer> nums) {
        int max = Integer.MIN_VALUE, min = Integer.MAX_VALUE;
        for(int num : nums) {
            max = Math.max(max, num);
            min = Math.min(min, num);
        }
        // exclude the cases where the second char has freq of 0
        // in our example s = "baaabbbaaaaaab", stringDeviationForTwoChars('a', 'c', s)
        // the max = 9, min = 1, we should return 0 instead of 8
        if(min > 0 || max < 0) return 0;
        return max - min;
    }
}
