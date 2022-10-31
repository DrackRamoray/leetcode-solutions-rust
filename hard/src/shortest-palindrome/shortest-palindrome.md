### [214. 最短回文串](https://leetcode.cn/problems/shortest-palindrome/)
给定一个字符串 s，你可以通过在字符串前面添加字符将其转换为回文串。找到并返回可以用这种方式转换的最短回文串。



##### 示例 1：
```
输入：s = "aacecaaa"
输出："aaacecaaa"
```

##### 示例 2：
```
输入：s = "abcd"
输出："dcbabcd"
```

##### 提示：
- 0 <= s.length <= 5 * 10<sup>4</sup>
- s 仅由小写英文字母组成

##### 题解：
```rust
impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let n = s.len();
        let mut next = vec![-1;n];

        for i in 1..n {
            let mut j = next[i-1];
            while j != -1 && s[(j+1) as usize..=(j+1) as usize] != s[i..=i] {
                j = next[j as usize];
            }
            if s[(j+1) as usize..=(j+1) as usize] == s[i..=i] {
                next[i] = j + 1;
            }
        }

        let mut j = -1;

        for i in (0..n).rev() {
            while j != - 1 && s[(j+1) as usize..=(j+1) as usize] != s[i..=i] {
                j = next[j as usize];
            }
            if s[(j+1) as usize..=(j+1) as usize] == s[i..=i] {
                j += 1;
            }
        }

        let tmp = if j == n as i32 - 1 {
            "".to_string()
        } else {
            (&s[j as usize+1..]).chars().rev().collect::<String>()
        };

        tmp + &s
    }
}
```
