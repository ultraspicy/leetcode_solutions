package binarysearch;

public class Solution1954 {
    public long minimumPerimeter(long neededApples) {
        long l = 1, r = 100000;
        while( l < r ) {
            long mid = (l + r) / 2;
            long target = 2 * mid * (1 + mid) * (2 * mid + 1);
            if (target >= neededApples) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        return 8 * l;
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
                    return target;
                } else {
                    start = mid + 1;
                }
            }
        }
        return -1;
    }
}
