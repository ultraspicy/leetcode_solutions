public class Solution1362 {
    public int[] closestDivisors(int num) {
        int[] ret1 = closetDivisor(num + 1);
        int absDiff = Math.abs(ret1[0] - ret1[1]);

        int[] ret2 = closetDivisor(num + 2);
        int absDiff2 = Math.abs(ret2[0] - ret2[1]);
        return absDiff < absDiff2 ? ret1 : ret2;
    }

    private int[] closetDivisor(int num) {
        int[] ret = new int[]{1, num};
        for(int i = (int)Math.sqrt(num); i > 1; i--) {
            if (num % i == 0) {
                ret = new int[]{num/i, i};
                break;
            }
        }
        return ret;
    }

    // lee's code
    public int[] closestDivisors_ByLee(int x) {
        for (int a = (int)Math.sqrt(x + 2); a > 0; --a) {
            if ((x + 1) % a == 0)
                return new int[]{a, (x + 1) / a};
            if ((x + 2) % a == 0)
                return new int[]{a, (x + 2) / a};
        }
        return new int[]{};
    }

}
