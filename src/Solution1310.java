public class Solution1310 {
    public int[] xorQueries(int[] arr, int[][] queries) {
        int[] prefix = new int[arr.length];
        int prev = 0;
        for (int i = 0; i < arr.length; i++) {
            prev ^= prev ^ arr[i];
            prefix[i] = prev;
        }

        int[] ret = new int[queries.length];
        for(int i = 0; i < queries.length; i++) {
            if(queries[i][0] == 0) {
                ret[i] = prefix[queries[i][1]];
            } else {
                ret[i] = prefix[queries[i][1]] ^ prefix[queries[i][0] - 1];
            }
        }
        return ret;
    }
}
