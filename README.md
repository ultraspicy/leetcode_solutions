# leetcode_solutions

###2551 https://leetcode.com/problems/put-marbles-in-bags/
* 划分 每个划分就是把相邻元素相加 然后排序

###2552 https://leetcode.com/problems/count-increasing-quadruplets/
* dp[i][j] - # of element are less than j in first i element
* 因为是permutation 限制了dp就是在0到n中进行

###2561 https://leetcode.com/problems/rearranging-fruits/
* 这个题想出来了 就是一个bug浪费很多时间 我们就swap一半 不是每个treemap的key都要完全swap 

###2565 https://leetcode.com/problems/subsequence-with-the-minimum-score/
* 这个题很难 完全没想出来

###2588 https://leetcode.com/problems/count-the-number-of-beautiful-subarrays/
* 想到dp了 没想到用map做cache

###2589 https://leetcode.com/problems/minimum-time-to-complete-all-tasks/
* 没想出来用number line的方法

###2594 https://leetcode.com/problems/minimum-time-to-repair-cars/description/
* 结果集上二分 因为很好确定最坏情况

###2597 https://leetcode.com/problems/the-number-of-beautiful-subsets/
* 去模 + house robber

### 2602 https://leetcode.com/problems/minimum-operations-to-make-all-array-elements-equal/description/
* 没想到用prefix sum + binary search的组合
* 调用Arrays.binarySearch(nums, 0, nums.length, target), toIndex是exclusive的 
* 注意一下int越界的问题，2 * 10^9, 2 ^ 32

### 2603 https://leetcode.com/problems/collect-coins-in-a-tree/  
* 有几个点没想到
  * 起始点不重要 在trim之后如果要回到原点 每条边必须经过两次
  * 这个题就是一个trim之后从leaf node往上走两次 看还剩下多少条边
