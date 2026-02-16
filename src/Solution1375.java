public class Solution1375 {
    public static void main(String[] args) {
        Solution1375 s = new Solution1375();

        System.out.println(s.numTimesAllBlue(new int[]{1,2,3,4,5,6}));
    }

    public int numTimesAllBlue(int[] light) {
        int i = 0, ret = 0, max = 0;
        while(i < light.length) {
            max = Math.max(max, light[i] - 1);
            if(max <= i++) {
                ret++;
            }
        }
        return ret;
    }
}
