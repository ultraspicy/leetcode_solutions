import java.util.HashMap;
import java.util.Map;

public class Soluton213 {
    public int maxNumberOfFamilies(int n, int[][] reservedSeats) {
        Map<Integer, Integer> m = new HashMap<>();
        for(int[] seat : reservedSeats) {
            m.put(seat[0], m.getOrDefault(seat[0], 0) + (1 << (seat[1] - 1)));
        }
        int ret = 2 * n;
        for(int i = 1; i <= n; i++) {
            if (m.containsKey(i)) ret -= helper(m.get(i));
        }
        return ret;
    }

    private int helper(int seat) {
        if ((seat & 510) == 0) {
            return 0;
        }
        if ((seat & 480) == 0 || (seat & 120) == 0 || (seat & 30) == 0) {
            return 1;
        }
        return 2;
    }
}
