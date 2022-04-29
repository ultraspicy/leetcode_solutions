import java.util.ArrayList;
import java.util.List;

public class Solution1382 {
    public TreeNode balanceBST(TreeNode root) {
        if (root == null) return root;
        List<Integer> list = new ArrayList<>();
        inOrder(root, list);

        return buildTree(list, 0, list.size() - 1);
    }

    private TreeNode buildTree(List<Integer> list, int from, int to) {
        if(from < 0 || to > list.size() || to < from) return null;

        int pivot = (from + to) / 2;

        TreeNode ret = new TreeNode(list.get(pivot));
        ret.left = buildTree(list, from , pivot - 1);
        ret.right = buildTree(list, pivot + 1, to);
        return ret;
    }

    private void inOrder(TreeNode root, List<Integer> list) {
        if (root == null) return;

        inOrder(root.left, list);
        list.add(root.val);
        inOrder(root.right, list);
    }
}
