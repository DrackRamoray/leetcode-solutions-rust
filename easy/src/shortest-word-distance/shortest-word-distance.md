### [243. 最短单词距离](https://leetcode.cn/problems/shortest-word-distance/)

##### 题解：
```rust
impl Solution {
    pub fn shortest_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
        let n = words_dict.len();
        let mut i = -1 * n as i32;
        let mut j = n as i32;
        let mut ans = n as i32;

        for k in 0..n {
            if words_dict[k] == word1 {
                i = k as i32;
            }
            if words_dict[k] == word2 {
                j = k as i32;
            }

            if i > j {
                ans = ans.min(i - j);
            } else {
                ans = ans.min(j - i);
            }
        }

        ans as i32
    }
}

```
