### [131. 分割回文串](https://leetcode.cn/problems/palindrome-partitioning/)
给你一个字符串 s，请你将 s 分割成一些子串，使每个子串都是 回文串 。返回 s 所有可能的分割方案。

回文串 是正着读和反着读都一样的字符串。



##### 示例 1：
```
输入：s = "aab"
输出：[["a","a","b"],["aa","b"]]
```

##### 示例 2：
```
输入：s = "a"
输出：[["a"]]
```

##### 提示：
- 1 <= s.length <= 16
- s 仅由小写英文字母组成

##### 题解：
```rust
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let n = s.len();
        let mut dp = vec![vec![true;n];n];

        for i in (0..n-1).rev() {
            for j in i+1..n {
                dp[i][j] = s[i..=i] == s[j..=j] && dp[i+1][j-1];
            }
        }

        let mut ans = vec![];

        Self::dfs(&mut ans, &mut vec![], &dp, &s, 0);
        
        ans
    }

    fn dfs(ans: &mut Vec<Vec<String>>, selected: &mut Vec<String>, dp: &Vec<Vec<bool>>, s: &str, begin: usize) {
        if begin == s.len() {
            ans.push(selected.to_vec());
            return;
        }

        for i in begin..s.len() {
            if dp[begin][i] {
                selected.push(s[begin..=i].to_string());
                Self::dfs(ans, selected, dp, s, i + 1);
                selected.pop();
            }
        }
    }
}
```

`回溯` `动态规划`
