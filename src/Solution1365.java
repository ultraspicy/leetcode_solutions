import java.util.Arrays;
import java.util.HashMap;
import java.util.Map;

public class Solution1365 {
    public int[] smallerNumbersThanCurrent(int[] nums) {
        Map<Integer, Integer> map = new HashMap<>();
        int[] cp = new int[nums.length];
        System.arraycopy(nums, 0, cp, 0, nums.length);
        Arrays.sort(cp);
        for (int i = 0; i < nums.length; i++) {
            if (i == 0 || cp[i - 1] != cp[i]) {
                map.put(cp[i], i);
            }
        }

        int[] ret = new int[nums.length];
        for (int i = 0; i < nums.length; i++) {
            ret[i] = map.get(nums[i]);
        }
        return ret;
    }
}
