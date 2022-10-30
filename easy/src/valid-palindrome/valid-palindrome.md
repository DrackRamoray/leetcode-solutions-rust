### [125. 验证回文串](https://leetcode.cn/problems/valid-palindrome/)
如果在将所有大写字符转换为小写字符、并移除所有非字母数字字符之后，短语正着读和反着读都一样。则可以认为该短语是一个 回文串 。

字母和数字都属于字母数字字符。

给你一个字符串 s，如果它是 回文串 ，返回 true ；否则，返回 false 。



##### 示例 1：
```
输入: s = "A man, a plan, a canal: Panama"
输出：true
解释："amanaplanacanalpanama" 是回文串。
```

##### 示例 2：
```
输入：s = "race a car"
输出：false
解释："raceacar" 不是回文串。
```

##### 示例 3：
```
输入：s = " "
输出：true
解释：在移除非字母数字字符之后，s 是一个空字符串 "" 。
由于空字符串正着反着读都一样，所以是回文串。
```

##### 提示：
- 1 <= s.length <= 2 * 10<sup>5</sup>
- s 仅由可打印的 ASCII 字符组成

##### 题解：
```rust
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut bytes = s.into_bytes();
        let mut lo = 0;
        let mut hi = bytes.len() - 1;

        while lo < hi {
            let t1 = bytes[lo].is_ascii_alphanumeric();
            let t2 = bytes[hi].is_ascii_alphanumeric();

            if t1 && t2 {
                if bytes[lo].to_ascii_lowercase() != bytes[hi].to_ascii_lowercase() {
                    return false;
                }

                lo += 1;
                hi -= 1;
            }

            if !t1 {
                lo += 1;
            }

            if !t2 {
                hi -= 1;
            }
        }

        true
    }
}
```

`双指针`
