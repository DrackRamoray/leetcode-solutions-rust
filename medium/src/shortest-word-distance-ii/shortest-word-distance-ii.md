### [244. 最短单词距离 II](https://leetcode.cn/problems/shortest-word-distance-ii/)

##### 题解：
```rust
struct WordDistance {
    words: Vec<String>,
}

impl WordDistance {

    fn new(wordsDict: Vec<String>) -> Self {
        Self {
            words: wordsDict,
        }
    }
    
    fn shortest(&self, word1: String, word2: String) -> i32 {
        let mut i = -1;
        let mut j = -1;
        let mut ans = self.words.len() as i32;

        for (n, w) in self.words.iter().enumerate() {
            if w == &word1 {
                i = n as i32;
            } else if w == &word2 {
                j = n as i32;
            }

            if i != - 1 && j != -1 {
                ans = ans.min((i - j).abs());
            }
        }

        ans
    }
}

```
