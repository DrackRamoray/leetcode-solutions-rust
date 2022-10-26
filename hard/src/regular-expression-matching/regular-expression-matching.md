### [10. 正则表达式匹配](https://leetcode.cn/problems/regular-expression-matching/)

给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。

- '.' 匹配任意单个字符
- '*' 匹配零个或多个前面的那一个元素
所谓匹配，是要涵盖 整个 字符串 s的，而不是部分字符串。

 

##### 示例 1：
```
输入：s = "aa", p = "a"
输出：false
解释："a" 无法匹配 "aa" 整个字符串。
```

##### 示例 2：
```
输入：s = "aa", p = "a*"
输出：true
解释：因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
```

##### 示例 3：
```
输入：s = "ab", p = ".*"
输出：true
解释：".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。
```

##### 提示：
- 1 <= s.length <= 20
- 1 <= p.length <= 30
- s 只包含从 a-z 的小写字母。
- p 只包含从 a-z 的小写字母，以及字符 . 和 *。
- 保证每次出现字符 * 时，前面都匹配到有效的字符

##### 题解：
```rust
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let ss = s.as_bytes();
        let pp = p.as_bytes();
        let n = ss.len();
        let m = pp.len();

        let mut dp = vec![vec![false;m+1];n+1];
        dp[0][0] = true;

        for i in 0..=n {
            for j in 1..=m {
                if pp[j-1] == b'*' {
                    dp[i][j] = dp[i][j] || dp[i][j-2]; // 没有匹配的情况
                    
                    if Self::matched(ss, pp, i, j - 1) { // 单个字符匹配的情况
                        dp[i][j] = dp[i][j] || dp[i-1][j]; // 多个字符匹配的情况
                    }
                } else {
                    if Self::matched(ss, pp, i, j) {
                        dp[i][j] = dp[i][j] || dp[i-1][j-1];
                    }
                }
            }
        }

        dp[n][m]
    }

    fn matched(s: &[u8], p: &[u8], i: usize, j: usize) -> bool {
        if i == 0 {
            return false;
        }

        if p[j-1] == b'.' {
            return true;
        }

        return s[i-1] == p[j-1];
    }
}
```

`动态规划`
