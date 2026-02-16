import java.util.Arrays;
import java.util.PriorityQueue;

public class Solution1383 {
    final int M = 1000000000 + 7;

    public int maxPerformance(int n, int[] speed, int[] efficiency, int k) {
        int[][] se = new int[speed.length][2];
        for(int i = 0; i < speed.length; i++) {
            se[i] = new int[]{speed[i], efficiency[i]};
        }
        Arrays.sort(se, (a, b) -> (b[1] - a[1]));
        PriorityQueue<Integer> pq = new PriorityQueue<>();
        long totalSpeed = 0, p = 0;
        for(int[] speedAndEff : se) {
            totalSpeed += (long)speedAndEff[0];
            pq.add(speedAndEff[0]);
            if (pq.size() > k) {
                totalSpeed -= pq.poll();
            }
            p = Math.max(p, totalSpeed * speedAndEff[1]);
        }
        return (int)(p % M);
    }
}
