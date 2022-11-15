### [409. 最长回文串](https://leetcode.cn/problems/longest-palindrome/)
给定一个包含大写字母和小写字母的字符串 s ，返回 通过这些字母构造成的 最长的回文串 。

在构造过程中，请注意 区分大小写 。比如 "Aa" 不能当做一个回文字符串。



##### 示例 1:
```
输入:s = "abccccdd"
输出:7
解释:
我们可以构造的最长的回文串是"dccaccd", 它的长度是 7。
```

##### 示例 2:
```
输入:s = "a"
输入:1
```

##### 示例 3：
```
输入:s = "aaaaaccc"
输入:7
```

##### 提示:
- 1 <= s.length <= 2000
- s 只由小写 和/或 大写英文字母组成

##### 题解：
```rust
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut cnt = vec![0;52];

        for ch in s.chars() {
            if ch.is_ascii_uppercase() {
                cnt[ch as usize - 65] += 1;
            } else {
                cnt[ch as usize - 71] += 1;
            }
        }

        let mut ans = 0;

        for num in cnt.iter() {
            ans += num / 2 * 2;
            if num % 2 == 1 && ans % 2 == 0 {
                ans += 1;
            }
        }

        ans
    }
}
```
