public class Solution1372 {
    public static void main(String[] args) {
        Solution1372 s = new Solution1372();

        System.out.println(4<< -1);
    }

    private int longestZigZag = 0;

    public int longestZigZag(TreeNode root) {
        helper(root, true);
        helper(root, false);
        return longestZigZag - 1;
    }

    private int helper(TreeNode node, boolean isLeftChild) {
        if (node == null) return 0;
        int l = helper(node.left, true), r = helper(node.right, false);
        longestZigZag = Math.max(longestZigZag, Math.max(l, r) + 1);
        return (isLeftChild ? r : l) + 1;
    }

    /**
     * lee's code
     */
//    public int longestZigZag(TreeNode root) {
//        return dfs(root)[2];
//    }
    private int[] dfs(TreeNode root) {
        if (root == null) return new int[]{-1, -1, -1};
        int[] left = dfs(root.left), right = dfs(root.right);
        int res = Math.max(Math.max(left[1], right[0]) + 1, Math.max(left[2], right[2]));
        return new int[]{left[1] + 1, right[0] + 1, res};
    }
}
