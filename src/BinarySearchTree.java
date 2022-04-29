public class BinarySearchTree {
    class TreeNode{
        int val;
        TreeNode left;
        TreeNode right;
        public TreeNode(int val) {
            this.val = val;
            this.left = this.right = null;
        }
    }

    public TreeNode searchBST(TreeNode root, int val) {
        if (root == null) {
            return null;
        }
        if (val < root.val) {
            return searchBST(root.left, val);
        } else if (val > root.val) {
            return searchBST(root.right, val);
        } else {
            return root;
        }
    }

    // assume it will still be a BST after change
    public void updateBST(TreeNode root, int target, int val) {
        if (root == null) {
            return;
        }
        if (target < root.val) {
            updateBST(root.left, target, val);
        } else if (target > root.val) {
            updateBST(root.right, target, val);
        } else {
            root.val = val;
        }
    }

    public TreeNode insertNode(TreeNode root, TreeNode node) {
        if (root == null) {
            return node;
        }
        if (root.val > node.val) {
            root.left = insertNode(root.left, node);
        } else {
            root.right = insertNode(root.right, node);
        }
        return root;
    }

    public TreeNode removeNode(TreeNode root, int value) {
        TreeNode dummy = new TreeNode(0);
        dummy.left = root;
        TreeNode parent = findParent(dummy, root, value);
        TreeNode node;
        if (parent.left != null && parent.left.val == value) {
            node = parent.left;
        } else if (parent.right != null && parent.right.val == value) {
            node = parent.right;
        } else {
            return dummy.left;
        }
        deleteNode(parent, node);
        return dummy.left;
    }

    private TreeNode findParent(TreeNode parent, TreeNode node, int value) {
        if (node == null) {
            return parent;
        }
        if (node.val == value) {
            return parent;
        }
        if (value < node.val) {
            return findParent(node, node.left, value);
        } else {
            return findParent(node, node.right, value);
        }
    }

    private void deleteNode(TreeNode parent, TreeNode node) {
        if (node.right == null) {
            if (parent.left == node) {
                parent.left = node.left;
            } else {
                parent.right = node.left;
            }
        } else {
            TreeNode temp = node.right;
            TreeNode father = node;
            while (temp.left != null) {
                father = temp;
                temp = temp.left;
            }
            if (father.left == temp) {
                father.left = temp.right;
            } else {
                father.right = temp.right;
            }
            if (parent.left == node) {
                parent.left = temp;
            } else {
                parent.right = temp;
            }
            temp.left = node.left;
            temp.right = node.right;
        }
    }
}
