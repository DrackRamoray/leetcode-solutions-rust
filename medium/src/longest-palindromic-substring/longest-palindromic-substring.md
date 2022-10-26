### [5. 最长回文子串](https://leetcode.cn/problems/longest-palindromic-substring/)

给你一个字符串 s，找到 s 中最长的回文子串。

##### 示例 1：
```
输入：s = "babad"
输出："bab"
解释："aba" 同样是符合题意的答案。
```

##### 示例 2：
```
输入：s = "cbbd"
输出："bb"
```

##### 提示：
- 1 <= s.length <= 1000
- s 仅由数字和英文字母组成

##### 题解：
```rust
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() < 2 {
            return s;
        }

        let len = s.len();
        let mut left = 0;
        let mut dp = vec![vec![false;len];len];
        let mut num = 1;

        for i in 0 ..len {
            dp[i][i] = true;
        }

        let sv = s.as_bytes();

        for k in 2..=len {
            for i in 0..len { // 枚举左边界
                let j = k + i - 1; // 右边界

                if j >= len {
                    break;
                }

                if sv[i] != sv[j] {
                    dp[i][j] = false;
                } else {
                    if j - i < 3 {
                        dp[i][j] = true;
                    } else {
                        dp[i][j] = dp[i+1][j-1];
                    }
                }

                if dp[i][j] && j - i + 1 > num {
                    num = j - i + 1;
                    left = i
                }
            }
        }

        s[left..left+num].into()
    }
}
```

`动态规划`

```rust
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let ss = s.as_bytes();
        let n = s.len();
        let m = 2 * n - 1;
        let mut ans = &ss[0..1];

        for i in 0..m {
            let mut left = i / 2;
            let mut right = i / 2 + i % 2;

            while left >= 0 && left < n && right < n && ss[left] == ss[right] {
                left -= 1;
                right += 1;
            }

            if right - left > ans.len() {
                ans = &ss[(left+1)..right];
            }
        }

        std::str::from_utf8(ans).unwrap().to_string()
    }
}
```

`中心扩展`
