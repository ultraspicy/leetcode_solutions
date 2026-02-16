public class Solution1379 {
    public final TreeNode getTargetCopy(final TreeNode original, final TreeNode cloned, final TreeNode target) {
        if (original == null) return null;
        if (original == target) return cloned;

        TreeNode ret = getTargetCopy(original.left, cloned.left, target);
        if(ret == null) {
            ret = getTargetCopy(original.right, cloned.right, target);
        }
        return ret;
    }
}
