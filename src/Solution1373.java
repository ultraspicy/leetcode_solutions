public class Solution1373 {
    public static void main(String[] args) {
        Solution1373 s = new Solution1373();

        System.out.println(4<< -1);
    }

    int maxSum = 0;
    public int maxSumBST(TreeNode root) {
        if (root == null) return 0;
        helper(root);
        return maxSum;
    }

    private int[] helper(TreeNode root) {
        if (root == null) {
            return new int[]{Integer.MAX_VALUE, Integer.MIN_VALUE, 1, 0};
        }

        int[] l = helper(root.left), r = helper(root.right);
        int isBST = 0, sum = l[3] + r[3] + root.val;
        if (l[1] < root.val && r[0] > root.val && l[2] == 1 && r[2] == 1) {
            isBST = 1;
            maxSum = Math.max(maxSum, sum);
        }
        return new int[]{root.left == null ? root.val : l[0], root.right == null ? root.val : r[1], isBST, sum};
    }
}
