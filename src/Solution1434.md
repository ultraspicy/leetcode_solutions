### Solution & Notes for [leetcode problem 1434](https://leetcode.com/problems/number-of-ways-to-wear-different-hats-to-each-other/)

#### Intuition
* the state is defined by the hats and people we already assigned. 
* some assignments are interchangable, i.e, assigning <h1:p1, h2:p2> is the same as assigning <h2:p1, h1:p2>, so memoization DP could be a good candidate
* a single person's assignment is meaningless, we need to know the overall assignment status, hereby we know we need a **bitMask** 
* hats = 40, people < 10. So it's better to do some reverse thinking, i.e. assigning people to hats. This shall condense the possible state from (10 * 2^40) to (40 * 2 ^ 10)
* upon all above, the state could be 
```
f = #(assigning the first ith hat with a bitMask state)
```
---------------

#### Mistakes
* Operator Precedence in Java: additive > shift > equality > bitwise. Sample code 
`if ((mask >> p & 1) == 1) continue;`
* `int[][] dp = new Integer[41][1 << n];` is problematic, as 0 has a special meaning here (no way to make further assignment). Then ` if (dp[i][mask] != 0) return dp[i][mask];` can result lots of duplicated calculation on the invalid path. Fix is 1) use `Integer, null` combo 2) still use int, but use -1 as default value

---------------

#### Grammar
* return the assigned value 
```
return dp[i][mask] = ret;
```