### [204. 计数质数](https://leetcode.cn/problems/count-primes/)
给定整数 n ，返回 所有小于非负整数 n 的质数的数量 。



##### 示例 1：
```
输入：n = 10
输出：4
解释：小于 10 的质数一共有 4 个, 它们是 2, 3, 5, 7 。
```

##### 示例 2：
```
输入：n = 0
输出：0
```

示例 3：
```
输入：n = 1
输出：0
```

##### 提示：
- 0 <= n <= 5 * 10<sup>6</sup>

##### 题解：
```rust
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut count = 0;

        for i in 2..n {
            count += Solution::is_prime(i);
        }

        count
    }

    fn is_prime(n: i32) -> i32 {
        let mut i = 2;

        while i * i <= n {
            if n % i == 0 {
                return 0;
            }

            i += 1;
        }

        1
    }
}
```