### [357. 统计各位数字都不同的数字个数](https://leetcode.cn/problems/count-numbers-with-unique-digits/)
给你一个整数 n ，统计并返回各位数字都不同的数字 x 的个数，其中 0 <= x < 10n 。


##### 示例 1：
```
输入：n = 2
输出：91
解释：答案应为除去 11、22、33、44、55、66、77、88、99 外，在 0 ≤ x < 100 范围内的所有数字。
```

##### 示例 2：
```
输入：n = 0
输出：1
```

##### 提示：
- 0 <= n <= 8

##### 题解：
```rust
impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        match n {
            0 => 1,
            _ => Self::count_numbers_with_unique_digits(n-1) + 9 * (11-n..10).product::<i32>(),
        }
    }
}
```
