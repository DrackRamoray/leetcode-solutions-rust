### [416. 分割等和子集](https://leetcode.cn/problems/partition-equal-subset-sum/)
给你一个 只包含正整数 的 非空 数组 nums 。请你判断是否可以将这个数组分割成两个子集，使得两个子集的元素和相等。



##### 示例 1：
```
输入：nums = [1,5,11,5]
输出：true
解释：数组可以分割成 [1, 5, 5] 和 [11] 。
```

##### 示例 2：
```
输入：nums = [1,2,3,5]
输出：false
解释：数组不能分割成两个元素和相等的子集。
```

##### 提示：
- 1 <= nums.length <= 200
- 1 <= nums[i] <= 100

##### 题解：
```rust
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let sum = nums.iter().sum::<i32>();

        if sum % 2 != 0 {
            return false;
        }

        let target = sum / 2;
        let &max_num = nums.iter().max().unwrap();

        if target < max_num {
            return false;
        }

        let n = nums.len();
        let target = target as usize;
        let mut dp = vec![vec![false;target + 1];n];

        for i in 0..n {
            dp[i][0] = true;
        }
        dp[0][nums[0] as usize] = true;

        for i in 1..n {
            let num = nums[i] as usize;

            for j in 1..=target {
                if j >= num {
                    dp[i][j] = dp[i-1][j] || dp[i-1][j-num];
                } else {
                    dp[i][j] = dp[i-1][j];
                }
            }
        }

        dp[n-1][target]
    }
}
```
