import java.util.HashSet;
import java.util.LinkedList;
import java.util.Queue;
import java.util.Set;

public class Solution1391 {
    public boolean hasValidPath(int[][] grid) {
        int[][][] directions = new int[][][] {
                {{0, -1}, {0, 1}},
                {{1, 0},  {-1, 0}},
                {{0, -1}, {1, 0}},
                {{0, 1},  {1, 0}},
                {{0, -1}, {-1, 0}},
                {{-1, 0}, {0, 1}},
        };

        int m = grid.length, n = grid[0].length;
        Queue<int[]> q = new LinkedList<>(); q.add(new int[]{0, 0});
        boolean[][] visited = new boolean[m][n]; visited[0][0] = true;

        while (!q.isEmpty()) {
            int[] c = q.poll();
            int x = c[0], y = c[1];
            if(x == m - 1 && y == n - 1) return true;
            int street = grid[x][y] - 1;
            for(int[] d : directions[street]) {
                int nx = x + d[0], ny = y + d[0];
                if (nx < 0 || nx >= m || ny < 0 || ny >= n) continue;
                int nextStreet = grid[nx][ny] - 1;
                for(int[] pd : directions[nextStreet]) {
                    if (nx + pd[0] == x && ny + pd[1] == y && !visited[nx][ny]) {
                        q.add(new int[]{nx, ny});
                        visited[nx][ny] = true;
                    }
                }
            }
        }
        return false;
    }
}
