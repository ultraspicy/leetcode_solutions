import java.util.*;

public class Solution1311 {

    public static void main(String[] args) {
        Solution1311 s = new Solution1311();

        s.watchedVideosByFriends(
                Arrays.asList(
                  Arrays.asList("A", "B"), Arrays.asList("C"), Arrays.asList("C", "B"), Arrays.asList("D")
                ),
                new int[][] {
                        {1, 2},
                        {0, 3},
                        {0, 3},
                        {1, 2},
                },
                0, 1
        );
    }

    public List<String> watchedVideosByFriends(List<List<String>> watchedVideos, int[][] friends, int id, int level) {
        List<Integer>[] graph = new List[friends.length];
        for(int i = 0; i < friends.length; i++) {
            graph[i] = new ArrayList<>();
        }
        for(int i = 0; i < friends.length; i++) {
            for (int friend : friends[i]) {
                graph[i].add(friend);
            }
        }

        Queue<Integer> q = new LinkedList<>(); q.add(id);
        Set<Integer> visited = new HashSet<>(); visited.add(id);

        while (level-- > 0) {
            int size = q.size();
            while(size-- > 0) {
                int v = q.poll();
                for (int neigh : graph[v]) {
                    if (visited.add(neigh)) {
                        q.add(neigh);
                    }
                }
            }
        }

        Map<String, Integer> m = new HashMap<>();
        while (!q.isEmpty()) {
            int v = q.poll();
            for(String video : watchedVideos.get(v)) {
                m.put(video, m.getOrDefault(video, 0) + 1);
            }
        }

        List<String> ans = new ArrayList<>(m.keySet());
        ans.sort((a, b) -> {
            int f1 = m.get(a), f2 = m.get(b);
            if(f1 != f2) {
                return f1 - f2;
            } else {
                return a.compareTo(b);
            }
        });
        return ans;
    }
}
