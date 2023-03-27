import java.util.*;

public class Solution {

    int M = 1000000000 + 7;

    public int findMinimumTime(int[][] tasks) {
        Arrays.sort(tasks, (o1, o2) -> {
            if (o1[1] -o2[1] == 0) return o1[0] - o2[0];
            return o1[1] - o2[1];
        });
        int[] done = new int[2001];

        int ret = 0;
        for(int i = 0; i < tasks.length; i++) {
            int[] task = tasks[i];
            int start = task[0], end = task[1], remain = task[2];
            // first calculate how many tasks have been done
            for(int j = end; j >= start && remain > 0; j--) {
                if(done[j] == -1) remain--;
            }
            // update number line
            for(int j = end; j >= start && remain > 0; j--) { {
                if(done[j] != -1) {
                    done[j] = -1;
                    ret++;
                    remain--;
                }
            }}
        }
        return ret;
    }

    public long repairCars(int[] ranks, int cars) {
        int c = 0;
        for(int rank : ranks) {
            c += Math.sqrt(1 / rank);
        }
        if (c >= cars) return 1;

        long l = 1, r = (long) ranks[0] * cars * cars;
        while (l + 1 < r) {
            long mid = (l + r) / 2;
            int capacity = 0;
            for(int rank : ranks) {
                capacity += Math.sqrt(mid / rank);
            }
            if (capacity >= cars) {
                r = mid;
            } else {
                l = mid;
            }
        }
        return r;
    }

    public long findScore(int[] nums) {
        TreeMap<Integer, List<Integer>> map = new TreeMap<>();
        for(int i = 0; i < nums.length; i++) {
            if (!map.containsKey(nums[i])) map.put(nums[i], new ArrayList<>());
            map.get(nums[i]).add(i);
        }

        long ret = 0;
        int remain = nums.length;
        while (remain > 0) {
            int firstKey = map.firstKey();
            for(int index : map.get(firstKey)) {
                if (nums[index] != -1) {
                    ret += nums[index];
                    nums[index] = -1;
                    remain--;
                    if (index > 0 && nums[index - 1] != -1){
                        nums[index - 1] = -1;
                        remain--;
                    }
                    if (index + 1 < nums.length || nums[index + 1] != -1) {
                        nums[index + 1] = -1;
                        remain--;
                    }
                }
            }
            map.remove(firstKey);
        }
        return ret;
    }

    public long minCost(int[] nums1, int[] nums2) {
        TreeMap<Integer, Integer> map = new TreeMap<>();
        for(int num : nums1) map.put(num, map.getOrDefault(num, 0) + 1);
        for(int num : nums2) map.put(num, map.getOrDefault(num, 0) - 1);
        int times = 0;
        for (int key : map.keySet()) {
            if (map.get(key) % 2 == 1) return -1;
            if (map.get(key) > 0) times += map.get(key);
        }
        long ret = 0, opTimes = 0, minKey = map.firstKey();
        while (opTimes < times) {
            int key = map.firstKey();
            if (map.get(key) != 0) {
                long rep = Math.min(Math.abs(map.get(key)), times - opTimes);
                ret += Math.min(rep * key / 2, minKey * rep);
                opTimes += rep;
            }
            map.remove(key);
        }
        return ret;
    }

    public int collectTheCoins(int[] coins, int[][] edges) {
        int n = coins.length;
        Set<Integer>[] adjs = new HashSet[n];
        for(int i = 0; i < n; i++) adjs[i] = new HashSet<>();
        for(int[] e : edges) {
            adjs[e[0]].add(e[1]);
            adjs[e[1]].add(e[0]);
        }

        int remain = n - 1;
        for(int i = 0; i < n; i++) {
            // a leaf node without coin
            if (adjs[i].size() == 1 && coins[i] == 0) {
                int toRemove = i;
                while(adjs[toRemove].size() == 1 && coins[toRemove] == 0) {
                    // modify its parent
                    Iterator<Integer> iter = adjs[toRemove].iterator();
                    int parent = iter.next();
                    adjs[parent].remove(toRemove);
                    // remove itself
                    adjs[toRemove] = new HashSet<>();
                    toRemove = parent;
                    remain--;
                }
            }
        }

        Queue<Integer> q = new LinkedList<>();
        for(int i = 0; i < n; i++) {
            if (adjs[i].size() == 1 && coins[i] == 1) {
                q.add(i);
            }
        }
        for (int loop = 0; loop < 2; loop++) {
            int size = q.size();
            for(int i = 0; i < size && remain > 0; i++) {
                int node = q.poll();
                int parent = adjs[node].iterator().next();
                adjs[parent].remove(node);
                adjs[node] = new HashSet<>();
                if(adjs[parent].size() == 1) {
                    q.add(parent);
                }
                remain--;
            }
        }

        return remain * 2;
    }

    public int minimumScore(String s, String t) {
        int[] dp = new int[t.length()];
        Arrays.fill(dp, -1);
        int i = s.length() - 1, j = t.length() - 1;
        while(i >= 0 && j >= 0) {
            if(s.charAt(i) == t.charAt(j)) {
                dp[j--] = i;
            }
            i--;
        }
        int res = j + 1, k = j + 1;
        for(i = 0, j = 0; i < s.length() && j < t.length() && res > 0; i++) {
            if(s.charAt(i) == t.charAt(j)) {
                while(k < t.length() && dp[k] <= i){
                    k++;
                }
                res = Math.min(res, k - j - 1);
                j++;
            }
        }
        return res;
    }

    public List<Long> minOperations(int[] nums, int[] queries) {
        Arrays.sort(nums);
        long[] prefix = new long[nums.length + 1];
        for(int i = 1; i <= nums.length; i++) {
            prefix[i] = prefix[i - 1] + nums[i - 1];
        }
        List<Long> ret = new ArrayList<Long>();
        for (int query : queries) {
            int pos = Arrays.binarySearch(nums, 0, nums.length, query);
            if (pos < 0) pos = -(pos + 1);
            long lessThanSum = prefix[pos];
            long greaterThanSum = prefix[prefix.length - 1] - prefix[pos];
            long op = (long) query * pos - lessThanSum + greaterThanSum - (nums.length - pos) * (long) query;
            ret.add(op);
        }
        return ret;
    }

    public static void main(String[] args) {
        Solution solution = new Solution();
        int[][] questions = new int[][]{
                {21,5},{92,3},{74,2},{39,4},{58,2},{5,5},{49,4},{65,3}
        };
        int[][] M = {{1,0,0},{0,1,0},{0,0,1}};
        String[] strings1 = new String[]{"dd","aa","bb","dd","aa","dd","bb","dd","aa","cc","bb","cc","dd","cc"}, strings2 = new String[]{"tack","act","acti"};

        String start = "hit", end = "cog";
        Set<String> dict = new HashSet<>(Arrays.asList("hot","dot","dog","lot","log"));
        //[[8,19,1],[3,20,1],[1,20,2],[6,13,3]]
        int[][] tasks = new int[][]{
                {6, 20, 3}
        };
        // [[0,1],[0,2],[1,3],[1,4],[2,5],[5,6],[5,7]]
        int[][] edges = new int[][]{{0,1}}; //{0,2},{1,3},{1,4},{2,5},{5,6},{5,7}
        System.out.println(solution.minOperations(new int[]{2,9,6,3}, new int[]{10}));
    }

    public List<Integer> largestDivisibleSubsetII(int[] nums) {
        List<Integer> ret = new ArrayList<>();
        if (nums == null || nums.length == 0) return ret;
        Map<Integer, Integer> factors = new HashMap<>();
        Map<Integer, Integer> prevs = new HashMap<>();
        for(int num : nums) factors.put(num, 1);
        for(int num : nums) prevs.put(num, -1);

        int lastNum = nums[0];
        for(int i = 0; i < nums.length; i++) {
            for(int factor : getFactors(nums[i])) {
                if(!factors.containsKey(factor)) {
                    continue;
                }
                if(factors.get(nums[i]) < factors.get(factor) + 1) {
                    factors.put(nums[i], factors.get(factor) + 1);
                    prevs.put(nums[i], factor);
                }
            }
            if (factors.get(nums[i]) > factors.get(lastNum)) {
                lastNum = nums[i];
            }
        }
        while(lastNum != -1) {
            ret.add(lastNum);
            lastNum = prevs.get(lastNum);
        }
        Collections.reverse(ret);
        return ret;
    }

    private List<Integer> getFactors(int num) {
        List<Integer> ret = new ArrayList<>();
        if(num == 1) return ret;
        for(int i = 1; i * i <= num; i++) {
            if (num % i == 0) {
                ret.add(i);
                if(i * i != num && i != 1) {
                    ret.add(num / i);
                }
            }
        }
        return ret;
    }

    public int longestIncreasingSubsequence(int[] nums) {
        if(nums == null || nums.length == 0) {
            return 0;
        }

        int[] LIS = new int[nums.length + 1];
        LIS[0] = Integer.MIN_VALUE;
        for(int i = 1; i < LIS.length; i++) {
            LIS[i] = Integer.MAX_VALUE;
        }

        int ret = 0;
        for(int i = 0; i < nums.length; i++) {
            int index = gte(LIS, nums[i]);
            LIS[index] = nums[i];
            ret = Math.max(ret , index);
        }
        return ret;
    }

    private int gte (int[] array, int target) {
        int start = 0, end = array.length - 1;
        while(start + 1 < end) {
            int mid = start + (end - start) / 2;
            if(array[mid] > target) {
                end = mid;
            } else if (array[mid] == target) {
                end = mid;
            } else {
                start = mid;
            }
        }
        if(array[start] >= target) return start;
        if(array[end] >= target) return end;
        return -1;
    }

    public int maxCoins(int[] nums) {
        if(nums == null || nums.length == 0) return 0;

        int len = nums.length;
        int[][] dp = new int[len][len];
        for(int i = 0; i < len; i++) {
            int left = (i == 0 ? 1 : nums[i - 1]), right = (i == len - 1 ? 1 : nums[i + 1]);
            dp[i][i] = left * right * nums[i];
        }

        for(int diff = 1; diff < len; diff++) {
            for(int i = 0, j = i + diff; i < len && j < len; i++, j++) {
                int left = (i == 0 ? 1 : nums[i - 1]), right = (j == len - 1 ? 1 : nums[j + 1]);
                for(int k = i + 1; k < j; k++) {
                    dp[i][j] = Math.max(dp[i][j], dp[i][k - 1] + dp[k + 1][j] + left * nums[k] * right);
                }
            }
        }
        return dp[0][len - 1];
    }

    public long permutationIndex(int[] A) {
        if (A == null || A.length == 0) {
            return 0;
        }
        long ret = 1;
        int factor = 1;
        for(int i = A.length - 1; i >= 0; i--) {
            if(i < A.length - 1) {
                factor = factor * (A.length - 1 - i);
            }
            int count = 0;
            for(int j = i + 1; j < A.length; j++) {
                if(A[j] < A[i]) count++;
            }

            ret += count * factor;
        }
        return ret;
    }

    public int[] nextPermutation(int[] nums) {
        int n = nums.length, j = n - 1;
        while(j > 0) {
            if(nums[j] > nums[j - 1]){
                break;
            }
            j--;
        }
        if(j == 0) {
            reverse(nums, 0, n - 1);
            return nums;
        }
        j = j - 1;
        int i = n - 1;
        while (i > 0) {
            if(nums[i] > nums[j]) {
                int tmp = nums[i];
                nums[i] = nums[j];
                nums[j] = tmp;
                reverse(nums, j + 1, nums.length - 1);
                break;
            }
            i--;
        }
        return nums;
    }

    private void reverse (int[] a, int from, int to) {
        if(from > a.length - 1) return;
        while (from < to) {
            int tmp = a[from];
            a[from++] = a[to];
            a[to--] = tmp;
        }
    }

    public int backPackV(int[] nums, int target) {
        // write your code here
        if (nums == null || nums.length == 0) {
            return 0;
        }

        int prefixSum = 0;
        int[][] dp = new int[nums.length + 1][target + 1];
        dp[0][0] = 1;
        for(int i = 1; i <= nums.length; i++) {
            prefixSum += nums[i - 1];
            dp[i][0] = 1;
            for(int j = 1; j <= Math.min(target, prefixSum); j++) {
                dp[i][j] = dp[i - 1][j];
                if(j >= nums[i - 1]) {
                    dp[i][j] += dp[i - 1][j - nums[i - 1]];
                }
            }
        }
        return dp[nums.length][target];
    }

    public int backPackII(int m, int[] A, int[] V) {
        if (m <= 0 || A == null || A.length == 0) return 0;

        int[][] dp = new int[A.length + 1][m + 1];
        for(int i = 1; i <= A.length; i++) {
            for(int j = 1; j <= m; j++) {
                dp[i][j] = dp[i - 1][j];
                if(j >= A[i - 1]) {
                    dp[i][j] = Math.max(dp[i][j], dp[i - 1][j - A[i - 1]] + V[i - 1]);
                }
            }
        }
        return dp[A.length][m];
    }

    public int backPack(int backpack, int[] A) {
        // write your code here
        int m = A.length, n = backpack;
        int[][] dp = new int[m + 1][n + 1];

        for(int i = 1; i <= m; i++) {
            for(int j = 1; j <= n; j++) {
                if(j >= A[i - 1]) {
                    dp[i][j] = Math.max(dp[i - 1][j - A[i - 1]] + A[i - 1], dp[i - 1][j]);
                } else {
                    dp[i][j] = dp[i - 1][j];
                }
            }
        }
        return dp[m][n];
    }

    public long kDistinctCharacters(String s, int k) {
        // Write your code here
        if(s == null || s.length() == 0) return 0;

        long ret = 0;
        int distinctChar = 0, i = 0, j = 0;
        int[] counter = new int[26];
        for(;i < s.length(); i++) {
            while(j < s.length() && distinctChar < k) {
                if (counter[s.charAt(j++) - 'a']++ == 0) {
                    distinctChar++;
                }
            }
            if(distinctChar >= k) {
                ret += (s.length() - j + 1);
            }
            if (--counter[s.charAt(i) - 'a'] == 0) {
                distinctChar--;
            }
        }
        return ret;
    }

    public int lengthOfLIS(int[] nums) {
        int[] dp = new int[nums.length];
        int len = 0;
        for(int i = 0; i < nums.length; i++) {
            int idx = Arrays.binarySearch(dp, 0, len, nums[i]);
            if(idx < 0) {
                idx = -(idx + 1);
                dp[idx] = nums[i];
            }
            if(idx == len) {
                len++;
            }
        }
        return len;
    }
    
    public List<Integer> postorderTraversal_morris(TreeNode root) {
        List<Integer> nums = new ArrayList<>();
        TreeNode cur = null;
        while (root != null) {
            if (root.right != null) {
                cur = root.right;
                while (cur.left != null && cur.left != root) {
                    cur = cur.left;
                }
                if (cur.left == root) {
                    cur.left = null;
                    root = root.left;
                } else {
                    nums.add(root.val);
                    cur.left = root;
                    root = root.right;
                }
            } else {
                nums.add(root.val);
                root = root.left;
            }
        }
        Collections.reverse(nums);
        return nums;
    }

    public List<Integer> preorderTraversal_morris(TreeNode root) {
        // morris traversal
        List<Integer> nums = new ArrayList<>();
        TreeNode cur = null;
        while (root != null) {
            if (root.left != null) {
                cur = root.left;
                // find the predecessor of root node
                while (cur.right != null && cur.right != root) {
                    cur = cur.right;
                }
                if (cur.right == root) {
                    cur.right = null;
                    root = root.right;
                } else {
                    nums.add(root.val);
                    cur.right = root;
                    root = root.left;
                }
            } else {
                nums.add(root.val);
                root = root.right;
            }
        }
        return nums;
    }

    public List<Integer> inorderTraversal_morris(TreeNode root) {
        List<Integer> nums = new ArrayList<>();
        TreeNode cur = null;

        while (root != null) {
            if (root.left != null) {
                cur = root.left;
                while (cur.right != null && cur.right != root) {
                    cur = cur.right;
                }

                if (cur.right == root) {
                    nums.add(root.val);
                    cur.right = null;
                    root = root.right;
                } else {
                    cur.right = root;
                    root = root.left;
                }
            } else {
                nums.add(root.val);
                root = root.right;
            }
        }

        return nums;
    }

    public List<Integer> postorderTraversal_iteration(TreeNode root) {
        List<Integer> list = new ArrayList<>();

        if (root == null) return list;

        Stack<TreeNode> stack = new Stack<>();
        stack.push(root);
        TreeNode prev = null;
        while(!stack.isEmpty()) {
            TreeNode curr = stack.peek();
            if(prev == null || prev.left == curr || prev.right == curr) {
                if (curr.left != null) {
                    stack.push(curr.left);
                } else if(curr.right != null) {
                    stack.push(curr.right);
                }
            } else if (curr.left == prev) {
                if (curr.right != null) {
                    stack.push(curr.right);
                }
            } else {
                stack.pop();
                list.add(curr.val);
            }
            prev = curr;
        }
        return list;
    }

    public double findMedianSortedArrays(int[] A, int[] B) {
        // write your code here
        int len = A.length + B.length;
        if(len % 2 == 1) {
            return findKth(A, B, 0, 0 , (len + 1) / 2);
        }
        return (
                    (double)findKth(A, B, 0, 0 , len / 2) +
                    (double)findKth(A, B, 0, 0 , len / 2 + 1)
                ) / 2;
    }

    private int findKth(int[] A, int[] B, int pa, int pb, int k) {
        if(pa == A.length) {
            return B[pb + k - 1];
        }
        if(pb == B.length) {
            return A[pa + k - 1];
        }
        if (k == 1) return Math.min(A[pa], B[pb]);

        if(k / 2 > A.length - pa) {
            return findKth(A, B, pa, pb + k / 2, k - k / 2);
        }
        if(k / 2 > B.length - pb) {
            return findKth(A, B, pa + k / 2, pb, k - k / 2);
        }

        int ea = A[pa + k / 2 - 1], eb = B[pb + k / 2 - 1];

        if(ea < eb) {
            return findKth(A, B, pa + k / 2, pb, k - k / 2);
        } else {
            return findKth(A, B, pa, pb + k / 2, k - k / 2);
        }
    }

    public int copyBooks(int[] pages, int k) {
        if (pages == null || pages.length == 0) return 0;

        int sum = 0, max = -1;
        for(int page : pages){
            sum += page;
            max = Math.max(max, page);
        }
        int start = max, end = sum;
        while(start + 1 < end) {
            int mid = start + (end - start) / 2;
            int copier = copier(pages, mid);
            if(copier > k) {
                // we need more copier, meaning pages to claim per copier is too small
                start = mid;
            } else {
                end = mid;
            }
        }
        return copier(pages, start) <= k ? start : end;
    }

    private int copier(int[] pages, int pagePerPerson) {
        int ret = 0, sum = 0;
        for(int i = 0; i < pages.length; i++) {
            sum += pages[i];
            if(sum > pagePerPerson) {
                ret++;
                i--;
                sum = 0;
            }
        }
        return ret + 1;
    }

    public int characterReplacement(String s, int k) {
        // write your code here
        if (s == null || s.length() == 0) return 0;

        int ret = 0, i = 0, j = 0, maxFreq = 0, toBeReplaced = 0;
        int[] counter = new int[26];

        while (i < s.length()) {
            // find last j that s[i, j - 1] needs to replace k char
            while(j < s.length() && toBeReplaced < k) {
                maxFreq = Math.max(maxFreq, ++counter[s.charAt(j++) - 'A']);
                toBeReplaced = j - i - maxFreq;
            }
            while(j < s.length() && counter[s.charAt(j) - 'A'] == maxFreq) {
                maxFreq = ++counter[s.charAt(j++) - 'A'];
            }
            ret = Math.max(ret, j - i);
            --counter[s.charAt(i) - 'A'];
            maxFreq = 0;
            for (int value : counter) {
                maxFreq = Math.max(maxFreq, value);
            }
            toBeReplaced = j - ++i - maxFreq;
        }
        return ret;
    }

    public int lengthOfLongestSubstringKDistinct(String s, int k) {
        if(s == null || s.length() == 0 || k == 0) return 0;
        // write your code here
        int[] counter = new int[100];
        int distinctChar = 0, i = 0, j = 0;
        int ret = 0;
        for(; i < s.length(); i++) {
            while(j < s.length() && distinctChar < k) {
                if(counter[s.charAt(j++) - 'A']++ == 0) {
                    distinctChar++;
                }
            }
            // stop at the next char s[i...j] has k distinct chars
            while(j < s.length() && counter[s.charAt(j) - 'A'] > 0) {
                counter[s.charAt(j) - 'A']++;
                j++;
            }
            ret = Math.max(ret, j - i);
            if(--counter[s.charAt(i) - 'A'] == 0) {
                distinctChar--;
            }
        }
        return ret;
    }

    public String repeatLimitedString(String s, int repeatLimit) {
        TreeMap<Character, Integer> occurrence = new TreeMap<>();
        for(char c : s.toCharArray()) {
            occurrence.put(c, occurrence.getOrDefault(c, 0) + 1);
        }

        StringBuilder sb = new StringBuilder();
        int len = sb.length();
        while(sb.length() < s.length()) {
            char last = occurrence.lastKey();
            int repeatTimes = Math.min(repeatLimit, occurrence.get(last));
            boolean findSeoncd = (repeatTimes != occurrence.get(last));

            if(repeatTimes == occurrence.get(last)) {
                occurrence.remove(last);
            } else {
                occurrence.put(last, occurrence.get(last) - repeatLimit);
            }
            while(repeatTimes-- > 0) {
                sb.append(last);
            }

            if (!findSeoncd) continue;

            Character second = occurrence.lowerKey(last);
            if (second == null) {
                break;
            }
            sb.append(second);
            if (occurrence.get(second) == 1) {
                occurrence.remove(second);
            } else {
                occurrence.put(second, occurrence.get(second) - 1);
            }
        }

        return sb.toString();
    }

    public long goodTriplets(int[] nums1, int[] nums2) {
        Set<Integer> prevs = new HashSet<>();
        int len = nums1.length;

        Map<Integer, Set<Integer>> toPrevs = new HashMap<>();
        for(int i = 0; i < len; i++) {
            toPrevs.put(nums1[i], new HashSet<>(prevs));
            prevs.add(nums1[i]);
        }

        Map<Integer, Set<Integer>> toPair = new HashMap<>();
        for(int i = 0; i < len; i++) {
            Set<Integer> prevsInNums1 = toPrevs.get(nums2[i]);
            Set<Integer> pair = new HashSet<>();
            for(int j = 0; j < i; j++) {
                if(prevsInNums1.contains(nums2[j])) {
                    pair.add(nums2[j]);
                }
            }
            toPair.put(nums2[i], pair);
        }
        long ret = 0;
        for(int key : toPair.keySet()) {
            Set<Integer> pair = toPair.get(key);
            for(int secondKey : pair) {
                ret += toPair.get(secondKey).size();
            }
        }
        return ret;
    }

    public void flatten(TreeNode root) {
        if(root == null) {
            return;
        }

        TreeNode next = null;
        if (root.right != null) {
            flatten(root.right);
            next = root.right;
        }
        if (root.left != null ) {
            flatten(root.left);
            TreeNode p = root.left;
            while(p.right != null) {
                p = p.right;
            }
            p.right = next;
            next = root.left;
        }
        root.left = null;
        root.right = next;
    }

     private String revert (String word) {
        return new StringBuilder(word).reverse().toString();
    }

    public int earliestFullBloom(int[] plantTime, int[] growTime) {

        class Tuple implements Comparable<Tuple> {
            int plant;
            int grow;
            int index;
            public Tuple(int plant, int grow, int index) {
                this.plant = plant;
                this.grow = grow;
                this.index = index;
            }

            @Override
            public int compareTo(Tuple o) {
                return o.grow - this.grow;
            }
        }

        List<Tuple> l = new ArrayList<>();
        for(int i = 0; i < plantTime.length; i++) {
            l.add(new Tuple(plantTime[i], growTime[i], i));
        }
        Collections.sort(l);

        int ret = 0, plantDay = 0;
        for(int i = 0; i < plantTime.length; i++) {
            plantDay += l.get(i).plant;
            ret = Math.max(ret, plantDay + l.get(i).grow);
        }
        return ret;
    }

    class Trie {
        char c;
        Trie[] children;
        boolean isLeaf;

        public Trie(char c) {
            this.c = c;
            children = new Trie[26];
            isLeaf = false;
        }
    }

    public int wordCount(String[] startWords, String[] targetWords) {
        Trie root = new Trie('*');
        startWords = preprocess(startWords);
        targetWords = preprocess(targetWords);

        int ret = 0;
        root = buildTrie(root, startWords);
        for (String word : targetWords) {
            if (fullScan(root, word)) {
                System.out.println(word);
                ret++;
            }
        }
        return ret;
    }

    private String[] preprocess(String[] words) {
        int n = words.length;
        for(int i = 0; i < n; i++) {
            char[] chars = words[i].toCharArray();
            Arrays.sort(chars);
            words[i] = String.valueOf(chars);
        }
        return words;
    }

    private boolean fullScan (Trie root, String word) {
        for (int i = 0; i < word.length(); i++) {
            String wordDeleteALetter = word.substring(0, i) + word.substring(i + 1, word.length());
            if (scan(root, wordDeleteALetter)) {
                return true;
            }
        }
        return false;
    }

    private boolean scan (Trie root, String word) {
        Trie p = root;
        for(int i = 0; i < word.length(); i++) {
            char c = word.charAt(i);
            if (p.children[c - 'a'] != null) {
                p = p.children[c - 'a'];
            } else {
                return false;
            }
        }
        return p.isLeaf;
    }

    private Trie buildTrie(Trie root, String[] words) {
        for(String word : words) {
            Trie p = root;
            for(char c : word.toCharArray()) {
                if (p.children[c - 'a'] == null) {
                    p.children[c - 'a'] = new Trie(c);
                }
                p = p.children[c - 'a'];
            }
            p.isLeaf = true;
        }
        return root;
    }

    public int minSwaps(int[] nums) {
        int len = nums.length, sum  = 0;
        int[] numsDouble = new int[len * 2];
        for(int i = 0; i < len; i++) {
            numsDouble[i] = numsDouble[i + len] = nums[i];
            if(nums[i] == 1) {
                sum++;
            }
        }
        int size = len - sum, zeroCount = 0;
        for(int i = 0; i < size; i++) {
            if(nums[i] == 0) zeroCount++;
        }
        int maxZeroCount = zeroCount;
        for(int i = 1; i < len; i++) {
            if (numsDouble[i + size - 1] == 1) {
                if(nums[i - 1] == 0) {
                    zeroCount--;
                }
            } else {
                if(nums[i - 1] == 1) {
                    zeroCount++;
                    maxZeroCount = Math.max(zeroCount, maxZeroCount);
                }
            }
        }
        return size - maxZeroCount;
    }

    public void sortColors(int[] nums) {
        quickSort(nums, 0, nums.length - 1);
    }

    private void quickSort(int[] nums, int l, int r) {
        if(l >= r) {
            return;
        }
        int pivot = r, i = l, j = r;

        while(i < j) {
            if(nums[i++] > nums[pivot]) {
                swap(nums, --i, --j);
            }
        }
        swap(nums, pivot, j);

        // the first half <= pivot
        quickSort(nums, l, i - 1);
        // the second half > pivot
        quickSort(nums, i + 1, r);
    }

    public int findKthLargest(int[] nums, int k) {
        int i = findKthLargest(nums, 0, nums.length - 1, k);
        return nums[i];
    }

    private int findKthLargest(int[] nums, int lo, int hi, int k) {
        int pivot = nums[hi], i = lo, j = hi;
        while(i < j) {
            if(nums[i++] <= pivot) {
                swap(nums, --i, --j);
            }
        }
        swap(nums, hi, j);
        int n = i - lo + 1;
        if(n == k) {
            return i;
        } else if(n > k) {
            return findKthLargest(nums, lo, i - 1, k);
        } else {
            return findKthLargest(nums, i + 1, hi, k - n);
        }

    }

    private void swap(int[] nums, int l, int r) {
        int tmp = nums[l];
        nums[l] = nums[r];
        nums[r] = tmp;
    }

    int min = Integer.MAX_VALUE;
    public int assignBikes(int[][] workers, int[][] bikes) {
        int m = workers.length, n = bikes.length;
        boolean[] visited = new boolean[n];
        assign(workers, bikes, visited, 0, 0);
        return min;
    }

    private int dis(int[] worker, int[] bikes) {
        return Math.abs(worker[0] - bikes[0]) + Math.abs(worker[1] - bikes[1]);
    }

    private void assign(int[][] workers, int[][] bikes, boolean[] visited, int idx, int sum) {
        if (idx == workers.length) {
            min = Math.min(min, sum);
            return;
        }

        for(int i = 0; i < bikes.length; i++) {
            if (!visited[i]) {
                visited[i] = true;
                assign(workers, bikes, visited, idx + 1, sum + dis(workers[idx], bikes[i]));
                visited[i] = false;
            }
        }
    }

    public int numPoints(int[][] points, int r) {
        int l = points.length;
        Map<String, Integer> m = new HashMap<>();

        for(int i = 0; i < l; i++) {
            int x1 = points[i][0], y1 = points[i][1];
            for (int j = i + 1; j < l; j++) {
                int x2 = points[j][0], y2 = points[j][1];
                for (int k = j + 1; k < l; k++) {
                    int x3 = points[k][0], y3 = points[k][1];
                    int a = getA(x1, x2, x3, y1, y2, y3), b = getB(x1, x2, x3, y1, y2, y3), c = getC(x1, x2, x3, y1, y2, y3);
                    double x = -(double)b/(2 * a), y = -(double)c/(2 * a);
                    String key = String.format("%.2f", x) + " " + String.format("%.2f", y);
                    m.put(key, m.getOrDefault(key, 0) + 1);
                }
            }
        }

        int max = 0;
        for(Map.Entry<String, Integer> v : m.entrySet()) {
            max = Math.max(max, v.getValue());
        }
        return max;
    }

    int getA(int x1, int x2, int x3, int y1, int y2, int y3) {
        return x1 * (y2 - y3) - y1 * (x2 - x3) + x2 * y3 - x3 * y2;
    }

    int getB(int x1, int x2, int x3, int y1, int y2, int y3) {
        return (x1 * x1 + y1 * y1) * (y3 - y2) + (x2 * x2 + y2 * y2) * (y1 - y3) + (x3 ^ x3 + y3 ^ y3) * (y2 - y1);
    }

    int getC(int x1, int x2, int x3, int y1, int y2, int y3) {
        return (x1 * x1 + y1 * y1) * (x2 - x3) + (x2 * x2 + y2 * y2) * (x3 - x1) + (x3 ^ x3 + y3 ^ y3) * (x1 - x2);
    }


    public int[] maxSlidingWindow(int[] nums, int k) {
        if(k < 0 || nums == null || nums.length == 0) return new int[0];
        int[] res = new int[nums.length - k + 1];
        Deque<Integer> deque = new ArrayDeque<>();

        for(int i = 0; i < nums.length; i++) {
            while(!deque.isEmpty() && deque.peek() < i - k + 1) {
                deque.poll();
            }
            while(!deque.isEmpty() && nums[deque.peekLast()] < nums[i]) {
                deque.pollLast();
            }

            deque.offer(i);
            if(i >= k - 1) {
                res[i - k + 1] = nums[deque.peek()];
            }
        }
        return res;
    }

    public int constrainedSubsetSum(int[] nums, int k) {
        Deque<Integer> d = new ArrayDeque<>();
        int[] rst = new int[nums.length];
        int ret = Integer.MIN_VALUE;
        for(int i = 0; i < nums.length; i++) {
            while(!d.isEmpty() && d.peek() < i - k) {
                d.poll();
            }
            rst[i] = nums[i] + (d.isEmpty() ? 0 : (Math.max(rst[d.peek()], 0)));
            while(!d.isEmpty() && rst[d.peekLast()] < rst[i]) d.pollLast();
            d.offer(i);
            ret = Math.max(rst[i], ret);
        }
        return ret;
    }

    public int constrainedSubsetSumDSF_TLE(int[] nums, int k) {
        int[] dp = new int[nums.length];
        Arrays.fill(dp, Integer.MIN_VALUE);
        int ret = dfs(dp, nums, k, 0);
        for(int i = 1; i < dp.length; i++) ret = Math.max(ret, dp[i]);

        return ret;
    }

    private int dfs(int[] dp, int[] nums, int k, int start) {
        if(start == nums.length - 1) {
            dp[start] = nums[nums.length - 1];
            return nums[nums.length - 1];
        }
        if(dp[start] != Integer.MIN_VALUE) return dp[start];

        int ret = nums[start];
        for(int i = start + 1; i <= start + k && i < nums.length; i++) {
            int sum = nums[start] + dfs(dp, nums, k, i);
            ret = Math.max(ret, sum);
        }
        dp[start] = ret;
        return ret;
    }

    private long cbn(long n, long r) {
        if (r == 0 || r == n) return 1;
        if (r > n) return 0;
        long rst =  1;
        for(long i = n; i > n -r; i--) {
            rst = rst * i;
        }
        for(long i = 2; i <= r; i++) {
            rst = rst / i;
        }
        return rst;
    }

    public String stoneGameIII(int[] piles) {
        int[] sum = new int[piles.length];

        int s = 0;
        for (int i = piles.length - 1; i >= 0; i--) {
            s += piles[i];
            sum[i] = s;
        }

        int[] dp = new int[piles.length];
        boolean[] visited = new boolean[piles.length];

        int alex = search(piles, dp, visited, sum, 0);
        int bob = sum[0] - alex;

        System.out.println("Alice: " + alex);
        System.out.println("Bob: " + bob);
        if (alex > bob) {
            return "Alice";
        } else if (bob > alex){
            return "Bob";
        } else {
            return "Tie";
        }
    }

    private int search(int[] piles, int[] dp, boolean[] visited, int[] sum, int l) {
        if (l >= piles.length) return 0;
        if (visited[l]) {
            return dp[l];
        }
        visited[l] = true;

        int t1 = search(piles, dp, visited, sum, l+1);
        int t2 = search(piles, dp, visited, sum, l+2);
        int t3 = search(piles, dp, visited, sum, l+3);
        dp[l] = sum[l] - Math.min(Math.min(t1, t2), t3);

        return dp[l];
    }
}
