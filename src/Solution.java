import java.util.*;

public class Solution {

    int M = 1000000000 + 7;

    public static void main(String[] args) {
        Solution solution = new Solution();
        System.out.println("");
    }

    public void wallsAndGates(int[][] rooms) {
        // figure out the gates
        List<int[]> gates = new ArrayList<>();
        int m = rooms.length, n = rooms[0].length;
        for(int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (rooms[i][j] == 0) {
                    gates.add(new int[]{i, j});
                }
            }
        }

        // for each gate, BFS and update the min distance to the gate
        int[][] directions = new int[][]{{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        for(int[] gate: gates) {
            // bfs for a single gate
            Queue<int[]> q = new LinkedList<>();
            q.add(gate);
            boolean[][] visited = new boolean[m][n];
            int distance = 1;
            while (!q.isEmpty()) {
                int size = q.size();
                while(size-- > 0) {
                    int[] head = q.poll();
                    for(int[] direction : directions) {
                        int newX = head[0] + direction[0], newY = head[1] + direction[1];
                        if(newX >= 0 && newX < m && newY >= 0 && newY < n && rooms[newX][newY] != 0 && rooms[newX][newY] != -1 && !visited[newX][newY]) {
                            rooms[newX][newY] = Math.min(distance, rooms[newX][newY]);
                            q.add(new int[]{newX, newY});
                            visited[newX][newY] = true;
                        }
                    }
                }
                distance++;
            }
        }
        // traverse the rooms to update the isolated rooms
        
    }
}