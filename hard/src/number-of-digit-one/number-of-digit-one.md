### [233. 数字 1 的个数](https://leetcode.cn/problems/number-of-digit-one/)
给定一个整数 n，计算所有小于等于 n 的非负整数中数字 1 出现的个数。



##### 示例 1：
```
输入：n = 13
输出：6
```

##### 示例 2：
```
输入：n = 0
输出：0
```

##### 提示：
- 0 <= n <= 10<sup>9</sup>

##### 题解：
```rust
impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let mut count = 0;
        let mut high = n / 10;
        let mut cur = n % 10;
        let mut low = 0;
        let mut digit = 1;

        while high != 0 || cur != 0 {
            match cur {
                0 => count += high * digit,
                1 => count += high * digit + low + 1,
                _ => count += (high + 1) * digit
            };

            low += cur * digit;
            cur = high % 10;
            high /= 10;
            digit *= 10;
        }

        count
    }
}
```
