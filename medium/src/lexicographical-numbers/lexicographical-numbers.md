### [386. 字典序排数](https://leetcode.cn/problems/lexicographical-numbers/)
给你一个整数 n ，按字典序返回范围 [1, n] 内所有整数。

你必须设计一个时间复杂度为 O(n) 且使用 O(1) 额外空间的算法。



##### 示例 1：
```
输入：n = 13
输出：[1,10,11,12,13,2,3,4,5,6,7,8,9]
```

##### 示例 2：
```
输入：n = 2
输出：[1,2]
```

##### 提示：
- 1 <= n <= 5 * 10<sup>4</sup>

##### 题解：
```rust
impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans = vec![];
        let mut i = 1;

        for _ in 0..n {
            ans.push(i);

            if i * 10 <= n {
                i *= 10;
            } else {
                while i % 10 == 9 || i + 1 > n {
                    i /= 10;
                }

                i += 1;
            }
        }

        ans
    }
}
```
