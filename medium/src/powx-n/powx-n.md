### [50. Pow(x, n)](https://leetcode.cn/problems/powx-n/)

实现 pow(x, n) ，即计算 x 的整数 n 次幂函数（即，xn ）。

##### 示例 1：
```
输入：x = 2.00000, n = 10
输出：1024.00000
```

##### 示例 2：
```
输入：x = 2.10000, n = 3
输出：9.26100
```

##### 示例 3：
```
输入：x = 2.00000, n = -2
输出：0.25000
解释：2-2 = 1/22 = 1/4 = 0.25
```

##### 提示：
- -100.0 < x < 100.0
- -2<sup>31</sup> <= n <= 2<sup>31</sup>-1
- -10<sup>4</sup> <= xn <= 10<sup>4</sup>

##### 题解：
```rust
impl Solution {
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1_f64;
        }

        let mut exp = n.abs() as u32;
        let mut ans = 1_f64;

        while exp > 0 {
            if exp % 2 == 1 {
                ans *= (x as f64);
            }

            x *= x;
            exp /= 2;
        }

        if n < 0 {
            return 1_f64 / ans;
        }

        ans
    }
}
```

`数学`
