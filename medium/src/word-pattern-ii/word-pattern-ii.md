### [291. 单词规律 II](https://leetcode.cn/problems/word-pattern-ii/)

给你一种规律 pattern 和一个字符串 s，请你判断 s 是否和 pattern 的规律相匹配。

如果存在单个字符到字符串的 双射映射 ，那么字符串 s 匹配 pattern ，即：如果pattern 中的每个字符都被它映射到的字符串替换，那么最终的字符串则为 s 。双射 意味着映射双方一一对应，不会存在两个字符映射到同一个字符串，也不会存在一个字符分别映射到两个不同的字符串。

 

##### 示例 1：
```
输入：pattern = "abab", s = "redblueredblue"
输出：true
解释：一种可能的映射如下：
'a' -> "red"
'b' -> "blue"
```

##### 示例 2：
```
输入：pattern = "aaaa", s = "asdasdasdasd"
输出：true
解释：一种可能的映射如下：
'a' -> "asd"
```

##### 示例 3：
```
输入：pattern = "aabb", s = "xyzabcxzyabc"
输出：false
```

##### 提示：
- 1 <= pattern.length, s.length <= 20
- pattern 和 s 由小写英文字母组成

##### 题解：
```rust
use std::collections::HashMap;

impl Solution {
    pub fn word_pattern_match(pattern: String, s: String) -> bool {
        let mut p2s = HashMap::new();
        let mut s2p = HashMap::new();

        Self::track_back(&pattern, &s, &mut p2s, &mut s2p)
    }

    fn track_back(p: &str, s: &str, p2s: &mut HashMap<String, String>, s2p: &mut HashMap<String, String>) -> bool {
        if p.len() == 0 {
            return s.len() == 0;
        }

        if s.len() == 0 {
            return p.len() == 0;
        }

        let c = p.get(0..=0).unwrap();

        for i in 1..=(s.len() - p.len() + 1) {
            let ss = s.get(0..i).unwrap();

            if let Some(ms) = p2s.get(c) {
                if ms == ss && Self::track_back(&p[1..], &s[i..], p2s, s2p)  {
                    return true;
                }
            } else if !s2p.contains_key(ss) {
                p2s.insert(c.to_string(), ss.to_string());
                s2p.insert(ss.to_string(), c.to_string());

                if Self::track_back(&p[1..], &s[i..], p2s, s2p) {
                    return true;
                }

                p2s.remove(c);
                s2p.remove(ss);
            }
        }

        false
    }
}
```
