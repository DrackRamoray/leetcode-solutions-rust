### [290. 单词规律](https://leetcode.cn/problems/word-pattern/)
给定一种规律 pattern 和一个字符串 s ，判断 s 是否遵循相同的规律。

这里的 遵循 指完全匹配，例如， pattern 里的每个字母和字符串 s 中的每个非空单词之间存在着双向连接的对应规律。



##### 示例1:
```
输入: pattern = "abba", s = "dog cat cat dog"
输出: true
```

##### 示例 2:
```
输入:pattern = "abba", s = "dog cat cat fish"
输出: false
```

##### 示例 3:
```
输入: pattern = "aaaa", s = "dog cat cat dog"
输出: false
```

##### 提示:
- 1 <= pattern.length <= 300
- pattern 只包含小写英文字母
- 1 <= s.length <= 3000
- s 只包含小写英文字母和 ' '
- s 不包含 任何前导或尾随对空格
- s 中每个单词都被 单个空格 分隔

##### 题解：
```rust
use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut p2s = HashMap::new();
        let mut s2p = HashMap::new();
        let mut pv = pattern.chars();
        let mut sv = s.split_whitespace();

        loop {
            match (pv.next(), sv.next()) {
                (Some(c), Some(w)) => {
                    if let Some(ww) = p2s.insert(c, w) {
                        if ww != w {
                            return false;
                        }
                    }

                    if let Some(cc) = s2p.insert(w, c) {
                        if cc != c {
                            return false;
                        }
                    }
                },
                (None, None) => break,
                _ => return false,
            }
        }

        true
    }
}
```
