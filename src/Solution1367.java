public class Solution1367 {
    public boolean isSubPath(ListNode head, TreeNode root) {
        if (head == null) return true;
        if(root == null) return false;

        if (root.val == head.val && (dfs(head.next, root.left) || dfs(head.next, root.right)) ) {
            return true;
        }
        return isSubPath(head, root.left) || isSubPath(head, root.right);
    }

    private boolean dfs (ListNode head, TreeNode root) {
        if (head == null) return true;
        if (root == null) return false;

        return root.val == head.val && (dfs(head.next, root.left) || dfs(head.next, root.right));
    }
}

