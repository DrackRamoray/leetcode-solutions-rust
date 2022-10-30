### [127. 单词接龙](https://leetcode.cn/problems/word-ladder/)

字典 wordList 中从单词 beginWord 和 endWord 的 转换序列 是一个按下述规格形成的序列 beginWord -> s<sub>1</sub> -> s<sub>2</sub> -> ... -> s<sub>k</sub>：

- 每一对相邻的单词只差一个字母。
- 对于 1 <= i <= k 时，每个 s<sub>i</sub> 都在 wordList 中。注意， beginWord 不需要在 wordList 中。
- s<sub>k</sub> == endWord

给你两个单词 beginWord 和 endWord 和一个字典 wordList ，返回 从 beginWord 到 endWord 的 最短转换序列 中的 单词数目 。如果不存在这样的转换序列，返回 0 。


##### 示例 1：
```
输入：beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]
输出：5
解释：一个最短转换序列是 "hit" -> "hot" -> "dot" -> "dog" -> "cog", 返回它的长度 5。
```

##### 示例 2：
```
输入：beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]
输出：0
解释：endWord "cog" 不在字典中，所以无法进行转换。
```

##### 提示：
- 1 <= beginWord.length <= 10
- endWord.length == beginWord.length
- 1 <= wordList.length <= 5000
- wordList[i].length == beginWord.length
- beginWord、endWord 和 wordList[i] 由小写英文字母组成
- beginWord != endWord
- wordList 中的所有字符串 互不相同

##### 题解：
```rust
use std::collections::HashSet;

impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut words = HashSet::new();

        for w in word_list {
            words.insert(w);
        }

        if !words.contains(&end_word) {
            return 0;
        }

        let mut step = 1;
        let mut q = vec![begin_word.to_string()];
        let mut vis = HashSet::new();
        vis.insert(begin_word);
        let characters = (b'a'..=b'z').map(char::from).collect::<Vec<_>>();

        while q.len() > 0 {
            let q_size = q.len();

            for _ in 0..q_size {
                let word = q.remove(0);

                for j in 0..word.len() {
                    for k in &characters {
                        let new_word = (&word[0..j]).to_string() + &(k.to_string()) + &word[j+1..];

                        if new_word == end_word {
                            return step + 1;
                        }

                        if !words.contains(&new_word) || new_word == word {
                            continue;
                        }

                        if !vis.contains(&new_word) {
                            q.push(new_word.to_string());
                            vis.insert(new_word);
                        }
                    }
                }
            }
            
            step += 1;
        }

        0
    }
}
```

`广度优先搜索`
