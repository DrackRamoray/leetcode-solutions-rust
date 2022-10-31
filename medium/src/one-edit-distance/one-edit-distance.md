### [161. 相隔为 1 的编辑距离](https://leetcode.cn/problems/one-edit-distance/)

##### 题解:
```rust
impl Solution {
    pub fn is_one_edit_distance(s: String, t: String) -> bool {
        if s.len() < t.len() {
            return Self::is_one_edit_distance(t, s);
        }

        if s.len() - t.len() > 1 {
            return false;
        }

        let s = s.as_bytes();
        let t = t.as_bytes();

        for i in 0..t.len() {
            if s[i] != t[i] {
                if s.len() == t.len() {
                    return s[i+1..] == t[i+1..];
                } else {
                    return s[i+1..] == t[i..];
                }
            }
        }

        s.len() == t.len() + 1
    }
}
```
