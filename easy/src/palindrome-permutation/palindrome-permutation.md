### [266. 回文排列](https://leetcode.cn/problems/palindrome-permutation/)
给定一个字符串，判断该字符串中是否可以通过重新排列组合，形成一个回文字符串。

##### 示例 1：
```
输入: "code"
输出: false
```

##### 示例 2：
```
输入: "aab"
输出: true
```

##### 示例 3：
```
输入: "carerac"
输出: true
```

##### 题解：
```rust
use std::collections::HashSet;

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        let mut ss = HashSet::new();

        for c in s.chars() {
            if ss.contains(&c) {
                ss.remove(&c);
            } else {
                ss.insert(c);
            }
        }

        ss.len() <= 1
    }
}
```
