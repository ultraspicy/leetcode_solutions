import java.util.*;

public class Solution1377 {

    public static void main(String[] args) {
        Solution1377 s = new Solution1377();
        System.out.println(s.frogPosition(
                3,
                new int[][]{
                        {2, 1},
                        {3, 2},
//                        {1,7},
//                        {2,4},
//                        {2,6},
//                        {3,5},
                },
                1,
                2
        ));
    }

    public double frogPosition(int n, int[][] edges, int t, int target) {
        if (n == 1) {
            return 1.0;
        }
        Map<Integer, List<Integer>> adj = new HashMap<>();

        for (int[] edge : edges) {
            int from = edge[0], to = edge[1];
            if (!adj.containsKey(from)) {
                adj.put(from, new ArrayList<>());
            }
            adj.get(from).add(to);
            if (!adj.containsKey(to)) {
                adj.put(to, new ArrayList<>());
            }
            adj.get(to).add(from);
        }

        double[] possibility = new double[n + 1]; possibility[1] = 1.0;
        Queue<Integer> q = new LinkedList<>(); q.add(1);
        boolean[] visited = new boolean[n + 1]; visited[1] = true;

        while (t-- > 0 && !q.isEmpty()) {
            int qSize = q.size();
            while (qSize-- > 0) {
                int head = q.poll();
                boolean hasUnvisited = false;
                for (int neigh: adj.get(head)) {
                    if (!visited[neigh]) {
                        hasUnvisited = true;
                        visited[neigh] = true;
                        q.add(neigh);
                        // if the spreading is not from root, minus 1
                        possibility[neigh] = possibility[head] / (adj.get(head).size() - (head == 1 ? 0 : 1));
                    }
                }
                if (hasUnvisited) {
                    possibility[head] = 0.0;
                }
            }

        }
        return possibility[target];
    }
}


