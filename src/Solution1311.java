import java.util.*;

public class Solution1311 {

    public static void main(String[] args) {
        Solution1311 s = new Solution1311();

        //System.out.println(s.longestValidParentheses(")()())"));
    }

    

    // public long minimumCost(int[] nums) {
    //     long sum = 0;
    //     for (int num : nums) sum += (long)num;
    
    // }

    // private int greatestNumSmallerThanOrEqualTo(int x) {
    //     char[] chars = String.valueOf(x).toCharArray();
    //     int firstHalf = Integer.parseInt(new String(chars, 0, chars.length / 2));
    //     // construct the number
    //     int ans;
    //     if (chars.length % 2 == 1) {
    //         ans = reconstruct(firstHalf, true, chars[chars.length / 2] - '0');
    //     } else {
    //         ans = reconstruct(firstHalf, false, -1);
    //     }
    //     if (ans <= x) return ans;

    //     if (chars.length % 2 == 1) {
    //         if(chars[chars.length / 2] == '0') {
    //             firstHalf--;
    //             return reconstruct(firstHalf, true, 9);
    //         } else {
    //             return reconstruct(firstHalf, true, chars[chars.length / 2] - '0' - 1);
    //         }
    //     } else {
    //         // deduct the first half
    //         firstHalf--;
    //         return reconstruct(firstHalf, false, -1);
    //     }
    // }

    // private int leastNumGreaterThanOrEqualTo(int x) {
    //     char[] chars = String.valueOf(x).toCharArray();
    //     int firstHalf = Integer.parseInt(new String(chars, 0, chars.length / 2));
    //     // construct the number
    //     int ans;
    //     if (chars.length % 2 == 1) {
    //         ans = reconstruct(firstHalf, true, chars[chars.length / 2] - '0');
    //     } else {
    //         ans = reconstruct(firstHalf, false, -1);
    //     }
    //     if (ans >= x) return ans;

    //     if (chars.length % 2 == 1) {
    //         if(chars[chars.length / 2] == '9') {
    //             firstHalf++;
    //             return reconstruct(firstHalf, true, 0);
    //         } else {
    //             return reconstruct(firstHalf, true, chars[chars.length / 2] - '0' + 1);
    //         }
    //     } else {
    //         firstHalf++;
    //         return reconstruct(firstHalf, false, -1);
    //     }
    // }

    // private int reconstruct (int num, boolean hasMid, int mid) {
    //     String reverse = new StringBuilder(String.valueOf(num)).reverse().toString();
    //     String midString = hasMid ? (mid + "") : "";
    //     return Integer.parseInt((num + "") + midString + reverse); 
    // }

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
