import java.util.Arrays;
import java.util.Comparator;
import java.util.PriorityQueue;

public class Solution857 {
    public double mincostToHireWorkers(int[] q, int[] w, int K) {
        double[][] rate = new double[q.length][2];
        for (int i = 0 ; i < q.length; i++) {
            rate[i] = new double[]{w[i]/q[i], q[i]};
        }
        Arrays.sort(rate, Comparator.comparingDouble(a -> a[0]));

        double ret = Double.MAX_VALUE, total = 0;
        PriorityQueue<Double> pq = new PriorityQueue<>((a, b) -> Double.compare(b, a));
        for(double[] rq : rate) {
            pq.add(rq[1]);
            total += rq[1];
            if(pq.size() > K) {
                total -= pq.poll();
            }
            if (pq.size() == K) ret = Math.min(ret, rq[0] * total);
        }
        return ret;
    }
}
