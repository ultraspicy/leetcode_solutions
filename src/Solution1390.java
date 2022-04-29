import java.util.HashSet;
import java.util.Set;

public class Solution1390 {
    public int sumFourDivisors(int[] nums) {
        int res = 0;

        for(int num : nums) {
            Set<Integer> set = helper(num);
            if (set.size() == 4) {
                for(int d : set) {
                    res += d;
                }
            }
        }
        return res;
    }

    private Set<Integer> helper(int num) {
        Set<Integer> set = new HashSet<>();
        for(int i = (int)Math.sqrt(num); i >= 1; i--) {
            if ((num % i) == 0) {
                set.add(num / i);
                set.add(i);
            }
        }
        return set;
    }
}
