### [367. 有效的完全平方数](https://leetcode.cn/problems/valid-perfect-square/)
给定一个 正整数 num ，编写一个函数，如果 num 是一个完全平方数，则返回 true ，否则返回 false 。

进阶：不要 使用任何内置的库函数，如  sqrt 。



##### 示例 1：
```
输入：num = 16
输出：true
```

##### 示例 2：
```
输入：num = 14
输出：false
```

##### 提示：
- 1 <= num <= 2<sup>31</sup> - 1

##### 题解：
```rust
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut x = num as f64;

        loop {
            let y = (x +  (num as f64) / x) / 2.0;

            if x - y < 1e-6 {
                break;
            }

            x = y;
        }

        return (x as i32).pow(2) == num;
    }
}
```
