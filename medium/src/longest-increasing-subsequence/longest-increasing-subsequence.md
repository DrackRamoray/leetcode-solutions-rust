### [300. 最长递增子序列](https://leetcode.cn/problems/longest-increasing-subsequence/)
给你一个整数数组 nums ，找到其中最长严格递增子序列的长度。

子序列 是由数组派生而来的序列，删除（或不删除）数组中的元素而不改变其余元素的顺序。例如，[3,6,2,7] 是数组 [0,3,1,6,2,2,7] 的子序列。


##### 示例 1：
```
输入：nums = [10,9,2,5,3,7,101,18]
输出：4
解释：最长递增子序列是 [2,3,7,101]，因此长度为 4 。
```

##### 示例 2：
```
输入：nums = [0,1,0,3,2,3]
输出：4
```

##### 示例 3：
```
输入：nums = [7,7,7,7,7,7,7]
输出：1
```

##### 提示：
- 1 <= nums.length <= 2500
- -104 <= nums[i] <= 10<sup>4</sup>


##### 进阶：
- 你能将算法的时间复杂度降低到 O(n log(n)) 吗?

##### 题解：
```rust
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut dp = vec![1;n];
        let mut ans = 1;

        for i in 0..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
            ans = ans.max(dp[i]);
        }

        ans
    }
}
```

```rust
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        
        if len < 1 {
            return 0
        }

        let mut dp = vec![i32::MAX;len+1];
        let mut count = 1;
        dp[count] = nums[0];

        for i in 1..len {
            if nums[i] > dp[count] {
                count = count + 1;
                dp[count] = nums[i];
            } else {
                let mut pos = 0;
                let mut l = 1;
                let mut r = len;

                while l <= r {
                    let m = (l + r) >> 1;

                    if nums[i] > dp[m] {
                        pos = m;
                        l = m + 1;
                    } else {
                        r = m - 1;
                    }
                }

                dp[pos + 1] = nums[i];
            }
        }

        count as i32
    }
}
```
