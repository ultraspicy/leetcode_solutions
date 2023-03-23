package segmentTree;

public class Solution307 {
    static class TreeNode {
        TreeNode left;
        TreeNode right;
        int val;
        public TreeNode(int val) {
            this.val = val;
        }

        public TreeNode buildSegmentTree (int[] nums, int s, int e) {
            if (s == e) {
                return new TreeNode(nums[s]);
            }
            int mid = (s + e) / 2;
            TreeNode left = buildSegmentTree(nums, s, mid);
            TreeNode right = buildSegmentTree(nums, mid + 1, e);
            TreeNode ret = new TreeNode(left.val + right.val);
            ret.left = left;
            ret.right = right;
            return ret;
        }

        public TreeNode updateNode(int index, int val, int s, int e) {
            if (index == s && index == e) {
                this.val = val;
                return this;
            }
            int mid = (s + e) / 2;
            if (index <= mid) {
                this.left = this.left.updateNode(index, val, s, mid);
            } else {
                this.right = this.right.updateNode(index, val, mid + 1, e);
            }
            this.val = this.left.val + this.right.val;
            return this;
        }

        // s - search range start
        // e - search range end
        // l - left range of the node
        // r - right range of the node
        public int rangeSum (int s, int e, int l, int r) {
            if (l == r || (s == l && e == r)) {
                return this.val;
            }

            int mid = (l + r) / 2;
            if (e <= mid) {
                return this.left.rangeSum(s, e, l, mid);
            } else if (s > mid) {
                return this.right.rangeSum(s, e, mid + 1, r);
            } else {
                return this.left.rangeSum(s, mid, l, mid) + this.right.rangeSum(mid + 1, e, mid + 1, r);
            }
        }
    }

    static class NumArray {

        TreeNode root;
        int len;

        public NumArray(int[] nums) {
            this.root = new TreeNode(0);
            this.len = nums.length;
            root = root.buildSegmentTree(nums, 0, nums.length - 1);
        }

        public void update(int index, int val) {
            root.updateNode(index, val, 0, len - 1);
        }

        public int sumRange(int left, int right) {
            return root.rangeSum(left, right, 0, len - 1);
        }
    }

    public static void main(String[] args) {
        NumArray numArray = new NumArray(new int[]{1, 3, 5});
        System.out.println(numArray.sumRange(0, 2));
        numArray.update(1, 2);
        System.out.println(numArray.sumRange(0, 2));
    }
}
