package interview2023;

import java.awt.*;
import java.util.*;
import java.util.List;

public class Solution {
    public static void main(String[] args) {
        Solution solution = new Solution();
        char[][] grid = new char[][]{{'1','1','1','1','0'},{'1','1','0','1','0'},{'1','1','0','0','0'},{'0','0','0','0','0'}};
        System.out.println(solution.countCompleteSubstrings("aaa" ,1));
    }

    public int countCompleteSubstrings(String word, int k) {
        String alpha = "abcdefghijklmnopqrstuvwxyz";

        int ret = 0, acc = 0;
        Map<Character, Integer> map = new HashMap<>();

        for(int i = 0; i < word.length(); i++) {
            char cur = word.charAt(i);
            char prev = i > 0 ? word.charAt(i - 1) : cur;

            if(Math.abs(alpha.indexOf(prev) - alpha.indexOf(cur)) > 2) {
                acc = 0;
                map = new HashMap<>();
                map.put(word.charAt(i), 1);
                continue;
            }
            // check the substring
            map.put(cur, map.getOrDefault(cur, 0) + 1);
            boolean ok = true;
            for(Character key : map.keySet()) {
                if(map.get(key) != k) {
                    ok = false;
                    break;
                }
            }
            if (ok) {
                acc = acc + 1;
                ret += (acc);
            }
        }
        return ret;
    }

    public long maxBalancedSubsequenceSum(int[] nums) {
        int[] phi = new int[nums.length];

        for(int i = 0; i < phi.length; i++) {
            phi[i] = nums[i] - i;
        }
        long ret = Long.MIN_VALUE;
        TreeMap<Integer, Long> map = new TreeMap<>();
        for(int i = 0; i < phi.length; i++) {
            if(nums[i] <= 0) {
                ret = Math.max(ret, nums[i]);
                continue;
            }

            long tmp = (long)nums[i];
            if (map.floorKey(phi[i]) != null) {
                tmp += map.get(map.floorKey(phi[i]));
            }
            while(map.ceilingKey(phi[i]) != null && map.get(map.ceilingKey(phi[i])) < tmp) {
                map.remove(map.ceilingKey(phi[i]));
            }
            map.put(phi[i], tmp);
            ret = Math.max(ret, tmp);
        }
        return ret;
    }

    public int[] leftmostBuildingQueries(int[] heights, int[][] queries) {
        int qn = queries.length, n = heights.length;
        int[] ret = new int[qn];
        Arrays.fill(ret, -1);
        List<int[]>[] groupedQueries = new ArrayList[n];
        for(int i = 0; i < n; i++) groupedQueries[i] = new ArrayList<>();
        Queue<int[]> queue = new PriorityQueue<>(Comparator.comparingInt(a -> a[0]));

        for(int i = 0; i < qn; i++) {
            int alice = queries[i][0], bob = queries[i][1];
            if (alice == bob) {
                ret[i] = alice;
            } else if (alice > bob && heights[alice] > heights[bob]) {
                ret[i] = alice;
            } else if(alice < bob && heights[alice] < heights[bob]) {
                ret[i] = bob;
            } else {
                // group query
                groupedQueries[Math.max(alice, bob)].add(new int[]{Math.max(heights[alice], heights[bob]), i});
            }
        }

        for(int i = 0; i < n; i++) {
            while(!queue.isEmpty() && queue.peek()[0] < heights[i]) {
                ret[queue.poll()[1]] = i;
            }

            for(int[] query: groupedQueries[i]) {
                queue.add(query);
            }
        }
        return ret;
    }

    public int search(int[] nums, int target) {
        // corner case num = [x]

        int start = 0, end = nums.length - 1;
        while (start < end) {
            int mid = (start + end) / 2;
            if(nums[mid] == target) return mid;
            if(nums[mid] < target) {
                if(nums[mid] < nums[start]) {
                    end = mid;
                } else {
                    start = mid + 1;
                }
            } else {
                // nums[mid] > target
                if(target > nums[start]) {
                    end = mid;
                } else if (target == nums[start]) {
                    return start;
                } else {
                    start = mid + 1;
                }
            }
        }
        return -1;
    }

    public int longestIncreasingPath(int[][] matrix) {
        int ret = 0;
        int[][] cache = new int[matrix.length][matrix[0].length];
        for(int i = 0; i < matrix.length; i++) {
            for(int j = 0; j < matrix[0].length; j++) {
                cache[i][j] = -1;
            }
        }
        for(int i = 0; i < matrix.length; i++) {
            for(int j = 0; j < matrix[0].length; j++) {
                ret = Math.max(ret, DFS(matrix, i, j, -1, cache));
            }
        }
        return ret;
    }

    private int DFS(int[][] matrix, int x, int y, int prev, int[][] cache) {
        if(x < 0 || x > matrix.length || y < 0 || y > matrix[0].length) {
            return 0;
        }
        if (matrix[x][y] < prev) {
            return 0;
        }
        if(cache[x][y] != -1) return cache[x][y];
        int[][] directions = new int[][]{{0, 1}, {0, -1}, {1, 0}, {-1, 0}};
        int deepest = 0;
        for(int[] direction : directions) {
            int nextX = x + direction[0], nextY = y + direction[1];
            int next = DFS(matrix, nextX, nextY, matrix[x][y], cache);
            deepest = Math.max(deepest, next);
        }
        cache[x][y] = deepest + 1;
        return deepest + 1;
    }

    public int minMeetingRooms(int[][] intervals) {
        Arrays.sort(intervals, (a, b) -> (a[0] == b[0] ? b[1] - a[1] : a[0] - b[0]));
        Queue<Integer> q = new PriorityQueue<>();
        int ret = 0;
        for(int[] internal : intervals) {
            while(!q.isEmpty() && q.peek() <= internal[0]) {
                q.poll();
            }
            q.add(internal[1]);
            ret = Math.max(q.size(), ret);
        }
        return ret;
    }

    public int minOperations(int n) {
        int prevOnes = 0;
        int ret = 0;
        List<Integer> list = new ArrayList<>();
        while(n > 0) {
            list.add(n % 2);
            n = n / 2;
        }
        for(int i = list.size() - 1; i - 3 >= 0; i--) {
            if(list.get(i) == 1 && list.get(i - 1) == 0 && list.get(i - 2) == 1 && list.get(i - 3) == 1) {
                list.remove(i - 1);
                list.add(i - 1, 1);
                ret++;
            }
            if(list.get(i) == 1 && list.get(i - 1) == 1 && list.get(i - 2) == 0 && list.get(i - 3) == 1) {
                list.remove(i - 2);
                list.add(i - 2, 1);
                ret++;
            }
        }
        for(int i = list.size() - 1; i >= 0; i--) {
            if(list.get(i) == 0) {
                if (prevOnes >= 2) {
                    ret += 2;
                } else if(prevOnes == 1) {
                    ret++;
                }
                prevOnes = 0;
            } else {
                prevOnes++;
            }
        }
        if (prevOnes >= 2) {
            ret += 2;
        } else if(prevOnes == 1) {
            ret++;
        }
        return ret;
    }

    public int lengthOfLongestSubstring(String s) {
        if(s == null || s.length() == 0) return 0;
        int start = 0, end = 0;
        int[] occurance = new int[26];
        int ret = 0;

        while(end < s.length()) {
            if(occurance[s.charAt(end) - 'a']++ > 0) {
                char dup = s.charAt(end);
                while(s.charAt(start) != dup) {
                    occurance[s.charAt(start++) - 'a']--;
                }
                occurance[s.charAt(start++) - 'a']--;
            }
            ret = Math.max(ret, end - start + 1);
            end++;
        }
        return ret;
    }

    public ListNode mergeKLists(ListNode[] lists) {
        Queue<ListNode> q = new PriorityQueue<>(Comparator.comparingInt(a -> a.val));
        for(ListNode n : lists) {
            if(n != null) {
                q.add(n);
            }
        }
        ListNode dummy = new ListNode(0), cur = dummy;
        while (!q.isEmpty()) {
            ListNode h = q.poll();
            cur.next = h;
            if (h.next != null) q.add(h.next);
            cur = cur.next;
        }
        return dummy.next;
    }

    public String alienOrder(String[] words) {
        Map<Character, Map<Character, Integer>> connections = new HashMap<>();
        Map<Character, Integer> inDegrees = new HashMap<>();
        // build dep
        for(int i = 0; i < words.length; i++) {
            String word1 = words[i];
            for(char c : word1.toCharArray()) recordInDegree(c, inDegrees);
            if (i == words.length - 1) continue;
            String word2 = words[i + 1];
            if(word2.startsWith(word1)) continue;
            if(word1.startsWith(word2)) return "";
            int ii = 0;
            while(word1.charAt(ii) == word2.charAt(ii)) ii++;
            buildConnection(connections, inDegrees, word1.charAt(ii), word2.charAt(ii));
        }
        // init q
        StringBuilder ret = new StringBuilder();
        Queue<Character> q = new LinkedList<>();
        for(char c : inDegrees.keySet()) {
            if (inDegrees.get(c) == 0) {
                q.add(c);
            }
        }
        // typo sort
        while(!q.isEmpty()) {
            char from = q.poll();
            ret.append(from);
            Map<Character, Integer> connection = connections.get(from);
            if (connection == null) continue;
            for(char to : connection.keySet()) {
                // deduct the connection (from->to) from total indegree(to)
                inDegrees.put(to, inDegrees.get(to) - connection.get(to));
                if (inDegrees.get(to) == 0) {
                    q.add(to);
                }
            }
        }
        if (ret.length() == inDegrees.keySet().size()) {
            return ret.toString();
        }
        return "";
    }

    private void buildConnection(
            Map<Character, Map<Character, Integer>> connections,
            Map<Character, Integer> inDegrees,
            char from,
            char to
    ) {
        if (from == to) return;

        inDegrees.put(to, inDegrees.getOrDefault(to, 0) + 1);
        connections
                .computeIfAbsent(from, k -> new HashMap<>())
                .put(to, connections.get(from).getOrDefault(to, 0) + 1);
    }

    private void recordInDegree (char c, Map<Character, Integer> inDegrees) {
        if (!inDegrees.containsKey(c)) {
            inDegrees.put(c, 0);
        }
    }
}
