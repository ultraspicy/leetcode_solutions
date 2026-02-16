import java.util.HashSet;
import java.util.LinkedList;
import java.util.Queue;
import java.util.Set;

public class Solution1368 {
    public static void main(String[] args) {
        int[][] grid = new int[][]{
                {1, 1, 1, 1},
                {2, 2, 2, 2},
                {1, 1, 1, 1},
                {2, 2, 2, 2}
        };
        Solution1368 s = new Solution1368();
        System.out.println(s.minCost(grid));
    }

    public int minCost(int[][] grid) {
        if (grid.length == 1 && grid[0].length == 1) return 0;
        Queue<int[]> q = new LinkedList<>(); q.add(new int[]{0, 0});
        Set<String> visited = new HashSet<>();
        int ret = 0;
        if (dfs(q, visited, grid, 0, 0)) {
            return 0;
        }
        while (!q.isEmpty()) {
            int size = q.size();
            ret++;
            for(int i = 0; i < size; i++) {
                int[] pos = q.poll();
                for(int act = 1; act <= 4; act++) {
                    grid[pos[0]][pos[1]] = act;
                    if (dfs(q, visited, grid, pos[0], pos[1])) return ret;
                }
            }
        }
        return ret;
    }

    private boolean dfs(Queue<int[]> q, Set<String> visited, int[][] grid, int x, int y) {
        int m = grid.length, n = grid[0].length;
        while (true) {
            switch (grid[x][y]) {
                case 1:
                    y++;
                    break;
                case 2:
                    y--;
                    break;
                case 3:
                    x++;
                    break;
                case 4:
                    x--;
                    break;
            }
            if(x == m - 1 && y == n - 1) {
                return true;
            }
            if (x >= 0 && y >= 0 && x < m && y < n && visited.add(x + " " + y)) {
                q.add(new int[]{x, y});
            } else {
                break;
            }
        }
        return false;
    }
}
