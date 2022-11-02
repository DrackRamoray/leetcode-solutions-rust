### [264. 丑数 II](https://leetcode.cn/problems/ugly-number-ii/)
给你一个整数 n ，请你找出并返回第 n 个 丑数 。

丑数 就是只包含质因数 2、3 和/或 5 的正整数。



##### 示例 1：
```
输入：n = 10
输出：12
解释：[1, 2, 3, 4, 5, 6, 8, 9, 10, 12] 是由前 10 个丑数组成的序列。
```

##### 示例 2：
```
输入：n = 1
输出：1
解释：1 通常被视为丑数。
```

##### 提示：
- 1 <= n <= 1690

##### 题解：
```rust
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![1];
        let mut i = 1;
        let mut i2 = 0;
        let mut i3 = 0;
        let mut i5 = 0;

        while i < n {
            dp.push((dp[i2] * 2).min(dp[i3] * 3).min(dp[i5] * 5));

            while dp[i2] * 2 <= dp[i] {
                i2 += 1;
            }

            while dp[i3] * 3 <= dp[i] {
                i3 += 1;
            }

            while dp[i5] * 5 <= dp[i] {
                i5 += 1;
            }

            i += 1;
        }

        dp[n-1]
    }
}
```
