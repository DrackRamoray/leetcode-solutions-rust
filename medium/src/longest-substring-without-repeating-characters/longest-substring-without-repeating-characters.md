### [3. 无重复字符的最长子串](https://leetcode.cn/problems/longest-substring-without-repeating-characters/)

给定一个字符串 s ，请你找出其中不含有重复字符的 最长子串 的长度。

##### 示例 1:
```
输入: s = "abcabcbb"
输出: 3 
解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
```

##### 示例 2:
```
输入: s = "bbbbb"
输出: 1
解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。
```

##### 示例 3:
```
输入: s = "pwwkew"
输出: 3
解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
     请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。
```

##### 提示：

- 0 <= s.length <= 5 * 10<sup>4<sup>
- s 由英文字母、数字、符号和空格组成

##### 题解：
```rust
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let n = s.len();
        let ss = s.as_bytes();
        let mut i = 0;
        let mut mp = std::collections::HashMap::new();
        let mut ans = 0;

        for j in 0..n {
            if let Some(k) = mp.get(&ss[j]) {
                i = i.max(k + 1);
            }

            ans = ans.max(j - i + 1);
            mp.insert(ss[j], j);
        }

        ans as i32
    }
}
```

`哈希` `滑动窗口`
