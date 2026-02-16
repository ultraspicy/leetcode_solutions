import java.util.*;

public class Solution1376 {
    public static void main(String[] args) {
        Solution1376 s = new Solution1376();

        System.out.println(s.numOfMinutes(
            15,
            0,
            new int[]{-1,0,0,1,1,2,2,3,3,4,4,5,5,6,6},
            new int[]{1,1,1,1,1,1,1,0,0,0,0,0,0,0,0}
        ));
    }

    public int numOfMinutes(int n, int headID, int[] manager, int[] informTime) {
        Map<Integer, List<Integer>> reportee = new HashMap<>();

        // build the map
        for (int i = 0; i < manager.length; i++) {
            if (manager[i] != -1) {
                if (!reportee.containsKey(manager[i])) reportee.put(manager[i], new ArrayList<>());
                reportee.get(manager[i]).add(i);
            }
        }

        // bfs
        int ret = 0;
        int[] inform = new int[n];
        Queue<Integer> q = new LinkedList<>();
        q.add(headID);
        while (!q.isEmpty()) {
            int head = q.poll();
            if(reportee.containsKey(head)) {
                for(int r : reportee.get(head)) {
                    q.add(r);
                    inform[r] = inform[head] + informTime[head];
                    ret = Math.max(ret, inform[r]);
                }
            }
        }
        return ret;
    }
}
