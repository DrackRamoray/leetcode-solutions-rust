### [132. 分割回文串 II](https://leetcode.cn/problems/palindrome-partitioning-ii/)

给你一个字符串 s，请你将 s 分割成一些子串，使每个子串都是回文。

返回符合要求的 最少分割次数 。



##### 示例 1：
```
输入：s = "aab"
输出：1
解释：只需一次分割就可将 s 分割成 ["aa","b"] 这样两个回文子串。
```

##### 示例 2：
```
输入：s = "a"
输出：0
```

##### 示例 3：
```
输入：s = "ab"
输出：1
```

##### 提示：
- 1 <= s.length <= 2000
- s 仅由小写英文字母组成

##### 题解：
```rust
impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len();
        let mut dp = vec![vec![true;n];n];

        for i in (0..n).rev() {
            for j in (i+1..n) {
                dp[i][j] = (s[i..=i] == s[j..=j]) && dp[i+1][j-1];
            }
        }

        let mut ans = vec![n;n];

        for i in 0..n {
            if dp[0][i] {
                ans[i] = 0;
            } else {
                for j in 0..i {
                    if dp[j+1][i] && ans[j] + 1 < ans[i] {
                        ans[i] = ans[j] + 1;
                    }
                }
            }
        }

        ans[n-1] as i32
    }
}
```

`动态规划`
