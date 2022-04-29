import java.lang.reflect.Array;
import java.util.*;

public class Solution1363 {
    public static void main(String[] args) {
        Solution1363 s = new Solution1363();

        System.out.println(s.largestMultipleOfThree(new int[]{0,0,0,0}));
    }

    public String largestMultipleOfThree(int[] digits) {
        int[] m1 = {1, 4, 7, 2, 5, 8}, m2 = {2, 5, 8, 1, 4, 7}, count = new int[10];
        int sum = 0;
        for(int d : digits) {
            sum += d;
            count[d]++;
        }
        while(sum % 3 != 0) {
            for(int d : (sum % 3 == 1 ? m1 : m2)) {
                if (count[d] > 0) {
                    count[d]--;
                    sum -= d;
                    break;
                }
            }
        }
        StringBuilder sb = new StringBuilder();
        for(int i = 9; i >= 0; i--) {
            sb.append(Character.toString('0' + i).repeat(count[i]));
        }
        return (sb.length() > 0 && sb.charAt(0) == '0') ? "0" : sb.toString();
    }
}
