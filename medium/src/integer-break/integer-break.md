### [343. 整数拆分](https://leetcode.cn/problems/integer-break/)
给定一个正整数 n ，将其拆分为 k 个 正整数 的和（ k >= 2 ），并使这些整数的乘积最大化。

返回 你可以获得的最大乘积 。



##### 示例 1:
```
输入: n = 2
输出: 1
解释: 2 = 1 + 1, 1 × 1 = 1。
```

##### 示例 2:
```
输入: n = 10
输出: 36
解释: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36。
```

##### 提示:
- 2 <= n <= 58

##### 题解：
```rust
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n <= 3 {
            return n - 1;
        }

        let mut n = n as usize;
        let mut dp = vec![0;n+1];
        
        for i in 2..=n {
            let mut max_value = 0;
            for j in 1..i {
                max_value = max_value.max(j * (i - j)).max(j * dp[i - j]);
            }
            dp[i] = max_value;
        }

        dp[n] as i32
    }
}
```

```rust
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n <= 3 {
            return n - 1;
        }

        let q = n as u32 / 3;
        let r = n % 3;

        if r == 0 {
            return 3_i32.pow(q);
        }

        if r == 1 {
            return 3_i32.pow(q - 1) * 4;
        }

        return 3_i32.pow(q) * 2
    }
}
```
