package binarysearch;

public class Solution1954 {
    public long minimumPerimeter(long neededApples) {
        long l = 1, r = 100000;
        while( l < r ) {
            long mid = (l + r) / 2;
            long target = 2 * mid * (1 + mid) * (2 * mid + 1);
            if (target >= neededApples) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        return 8 * l;
    }
}
