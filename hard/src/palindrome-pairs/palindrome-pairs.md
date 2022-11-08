### [336. 回文对](https://leetcode.cn/problems/palindrome-pairs/)
给定一组 互不相同 的单词， 找出所有 不同 的索引对 (i, j)，使得列表中的两个单词， words[i] + words[j] ，可拼接成回文串。



##### 示例 1：
```
输入：words = ["abcd","dcba","lls","s","sssll"]
输出：[[0,1],[1,0],[3,2],[2,4]]
解释：可拼接成的回文串为 ["dcbaabcd","abcddcba","slls","llssssll"]
```

##### 示例 2：
```
输入：words = ["bat","tab","cat"]
输出：[[0,1],[1,0]]
解释：可拼接成的回文串为 ["battab","tabbat"]
```

##### 示例 3：
```
输入：words = ["a",""]
输出：[[0,1],[1,0]]
```

##### 提示：
- 1 <= words.length <= 5000
- 0 <= words[i].length <= 300
- words[i] 由小写英文字母组成

##### 题解： 
```rust
use std::collections::HashMap;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mp = words
            .iter()
            .enumerate()
            .map(|(i, word)| (word.clone(), i))
            .collect::<HashMap<_, _>>();

        let mut ans = Vec::new();

        for (i, word) in words.iter().enumerate() {
            for k in 0..word.len() {
                if (0..=k / 2).all(|l| word[l..=l] == word[k-l..=k-l]) {
                    if let Some(&j) = mp.get(&(word[k+1..].chars().rev().collect::<String>())) {
                        ans.push(vec![j as i32, i as i32]);
                    }
                }
            }

            let rev = word.chars().rev().collect::<String>();
            if let Some(&j) = mp.get(&rev) {
                if i != j {
                    ans.push(vec![i as i32, j as i32]);
                }
            }
 
            for k in 0..rev.len() {
                if (0..=k/2).all(|l| rev[l..=l] == rev[k-l..=k-l]) {
                    if let Some(&j) = mp.get(&rev[k+1..]) {
                        ans.push(vec![i as i32, j as i32]);
                    }
                }
            }
        }

        ans
    }
}
```
