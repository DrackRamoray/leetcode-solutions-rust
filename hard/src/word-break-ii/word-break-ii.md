### [140. 单词拆分 II](https://leetcode.cn/problems/word-break-ii/)
给定一个字符串 s 和一个字符串字典 wordDict ，在字符串 s 中增加空格来构建一个句子，使得句子中所有的单词都在词典中。以任意顺序 返回所有这些可能的句子。

注意：词典中的同一个单词可能在分段中被重复使用多次。



##### 示例 1：
```
输入:s = "catsanddog", wordDict = ["cat","cats","and","sand","dog"]
输出:["cats and dog","cat sand dog"]
```

##### 示例 2：
```
输入:s = "pineapplepenapple", wordDict = ["apple","pen","applepen","pine","pineapple"]
输出:["pine apple pen apple","pineapple pen apple","pine applepen apple"]
解释: 注意你可以重复使用字典中的单词。
```

##### 示例 3：
```
输入:s = "catsandog", wordDict = ["cats","dog","sand","and","cat"]
输出:[]
```

##### 提示：
- 1 <= s.length <= 20
- 1 <= wordDict.length <= 1000
- 1 <= wordDict[i].length <= 10
- s 和 wordDict[i] 仅有小写英文字母组成
- wordDict 中所有字符串都 不同

##### 题解：
```rust
use std::collections::HashSet;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut solutions = Vec::new();
        let mut solution = Vec::new();
        let g = word_dict.into_iter().collect::<HashSet<_>>();

        Self::dfs(&s, &g, &mut solutions, &mut solution, 0);

        solutions
    }

    fn dfs(s: &str, g: &HashSet<String>, solutions: &mut Vec<String>, solution: &mut Vec<String>, pos: usize) {
        let n = s.len();

        if pos == n {
            solutions.push(solution.join(" "));
            return;
        }

        for i in (pos + 1)..=n {
            let w = s[pos..i].to_string();
            
            if (g.contains(&w)) {
                solution.push(w);
                Self::dfs(s, g, solutions, solution, i);
                solution.pop();
            }
        }
    }
}
```
