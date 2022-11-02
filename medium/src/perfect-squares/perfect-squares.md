### [279. 完全平方数](https://leetcode.cn/problems/perfect-squares/)
给你一个整数 n ，返回 和为 n 的完全平方数的最少数量 。

完全平方数 是一个整数，其值等于另一个整数的平方；换句话说，其值等于一个整数自乘的积。例如，1、4、9 和 16 都是完全平方数，而 3 和 11 不是。



##### 示例 1：
```
输入：n = 12
输出：3
解释：12 = 4 + 4 + 4
```

##### 示例 2：
```
输入：n = 13
输出：2
解释：13 = 4 + 9
```

##### 提示：
- 1 <= n <= 10<sup>4</sup>

##### 题解：
```rust
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0;n + 1];

        for i in 1..=n {
            let mut cnt = i32::MAX;
            let mut j = 1;

            while j * j <= i {
                cnt = cnt.min(dp[i - j * j]);
                j += 1;
            }

            dp[i] = cnt + 1;
        }

        dp[n]
    }
}
```
