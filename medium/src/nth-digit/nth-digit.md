### [400. 第 N 位数字](https://leetcode.cn/problems/nth-digit/)
给你一个整数 n ，请你在无限的整数序列 [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...] 中找出并返回第 n 位上的数字。



##### 示例 1：
```
输入：n = 3
输出：3
```

##### 示例 2：
```
输入：n = 11
输出：0
解释：第 11 位数字在序列 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ... 里是 0 ，它是 10 的一部分。
```

##### 提示：
- 1 <= n <= 2<sup>31</sup> - 1

##### 题解：
```rust
impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut start = 1;
        let mut digit = 1;
        let mut count = 9;
        let mut mn = n as i64;

        while mn > count {
            mn = mn - count;
            digit =  digit + 1;
            start = start * 10;
            count = 9 * start * digit;
        }

        let num = start + (mn - 1) / digit;
        let index = (mn - 1) % digit;
        let s = num.to_string().chars().nth(index as usize).unwrap();

        return s.to_digit(10).unwrap() as i32;
    }
}
```
