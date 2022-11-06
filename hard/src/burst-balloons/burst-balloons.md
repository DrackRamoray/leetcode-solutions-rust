### [312. 戳气球](https://leetcode.cn/problems/burst-balloons/)
有 n 个气球，编号为0 到 n - 1，每个气球上都标有一个数字，这些数字存在数组 nums 中。

现在要求你戳破所有的气球。戳破第 i 个气球，你可以获得 nums[i - 1] * nums[i] * nums[i + 1] 枚硬币。 这里的 i - 1 和 i + 1 代表和 i 相邻的两个气球的序号。如果 i - 1或 i + 1 超出了数组的边界，那么就当它是一个数字为 1 的气球。

求所能获得硬币的最大数量。



##### 示例 1：
```
输入：nums = [3,1,5,8]
输出：167
解释：
nums = [3,1,5,8] --> [3,5,8] --> [3,8] --> [8] --> []
coins =  3*1*5    +   3*5*8   +  1*3*8  + 1*8*1 = 167
```

##### 示例 2：
```
输入：nums = [1,5]
输出：10
```

##### 提示：
- n == nums.length
- 1 <= n <= 300
- 0 <= nums[i] <= 100

##### 题解：
```rust
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut coins = vec![1;n+2];
        coins[1..=n].clone_from_slice(&nums);
        let mut dp = vec![vec![0;n+2];n+2];

        for i in (0..=n).rev() {
            for j in i+1..=n+1 {
                for k in i+1..j {
                    dp[i][j] = dp[i][j].max(dp[i][k] + coins[i] * coins[k] * coins[j] + dp[k][j]);
                }
            }
        }

        dp[0][n+1]
    }
}
```
