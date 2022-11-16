### [424. 替换后的最长重复字符](https://leetcode.cn/problems/longest-repeating-character-replacement/)
给你一个字符串 s 和一个整数 k 。你可以选择字符串中的任一字符，并将其更改为任何其他大写英文字符。该操作最多可执行 k 次。

在执行上述操作后，返回包含相同字母的最长子字符串的长度。



##### 示例 1：

输入：s = "ABAB", k = 2
输出：4
解释：用两个'A'替换为两个'B',反之亦然。
##### 示例 2：

输入：s = "AABABBA", k = 1
输出：4
解释：
将中间的一个'A'替换为'B',字符串变为 "AABBBBA"。
子串 "BBBB" 有最长重复字母, 答案为 4。


##### 提示：

1 <= s.length <= 10<sup>5</sup>
s 仅由大写英文字母组成
0 <= k <= s.length

##### 题解：
```rust
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut cnt = vec![0;26];
        let ss = s.as_bytes();
        let n = ss.len();
        let mut left = 0;
        let mut right = 0;
        let mut ans = 0;

        while right < n {
            let index = ss[right] as usize - b'A' as usize;
            cnt[index] += 1;
            ans = ans.max(cnt[index]);

            if right as i32 - left as i32 + 1 - ans > k {
                cnt[ss[left] as usize - b'A' as usize] -= 1;
                left += 1;
            }

            right += 1;
        }

        (right - left) as i32
    }
}
```
