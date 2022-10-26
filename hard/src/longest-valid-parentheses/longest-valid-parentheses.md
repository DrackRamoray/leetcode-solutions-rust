### [32. 最长有效括号](https://leetcode.cn/problems/longest-valid-parentheses/)

给你一个只包含 '(' 和 ')' 的字符串，找出最长有效（格式正确且连续）括号子串的长度。

##### 示例 1：
```
输入：s = "(()"
输出：2
解释：最长有效括号子串是 "()"
```

##### 示例 2：
```
输入：s = ")()())"
输出：4
解释：最长有效括号子串是 "()()"
```

##### 示例 3：
```
输入：s = ""
输出：0
```

##### 提示：
- 0 <= s.length <= 3 * 10<sup>4</sup>
- s[i] 为 '(' 或 ')'

##### 题解：
```rust
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut ans = 0;
        let mut stack = vec![];
        stack.push(-1);
        let n = s.len();
        let ss = s.as_bytes();

        for i in 0..n {
            if ss[i] == b'(' {
                stack.push(i as i32);
            } else {
                stack.pop();

                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    ans = ans.max(i as i32 - stack[stack.len()-1]);
                }
            }
        }

        ans
    }
}
```

`栈`

```rust
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut ans = 0;
        let mut n = s.len();
        let ss = s.as_bytes();
        let mut dp = vec![0;n];

        for i in 1..n {
            if ss[i] == b')' {
                if ss[i-1] == b'(' {
                    dp[i] = if i >= 2 {
                        dp[i-2] + 2
                    } else {
                        2
                    };
                } else if i > dp[i-1] && ss[i-dp[i-1]-1] == b'(' {
                    dp[i] = dp[i-1] + if i >= dp[i-1] + 2 {
                        dp[i-dp[i-1]-2] + 2
                    } else {
                        2
                    };
                }

                ans = ans.max(dp[i]);
            }
        }

        ans as i32
    }
}
```

`动态规划`
