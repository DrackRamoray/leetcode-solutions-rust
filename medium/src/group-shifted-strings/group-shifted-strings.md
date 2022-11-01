### [249. 移位字符串分组](https://leetcode.cn/problems/group-shifted-strings/)


##### 题解：
```rust
use std::collections::HashMap;

impl Solution {
    pub fn group_strings(strings: Vec<String>) -> Vec<Vec<String>> {
        let mut ans = HashMap::new();

        for s in strings.into_iter() {
            let mut key = vec![];
            let bytes = s.as_bytes();
            let byte_1st = bytes[0];
            for byte in bytes {
                key.push((byte - byte_1st + 26) % 26);
            }

            let entry = ans.entry(key).or_insert(vec![]);
            (*entry).push(s);
        }

        ans.into_values().collect::<Vec<Vec<String>>>()
    }
}
```
