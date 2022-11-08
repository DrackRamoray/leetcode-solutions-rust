### [345. 反转字符串中的元音字母](https://leetcode.cn/problems/reverse-vowels-of-a-string/)
给你一个字符串 s ，仅反转字符串中的所有元音字母，并返回结果字符串。

元音字母包括 'a'、'e'、'i'、'o'、'u'，且可能以大小写两种形式出现。



##### 示例 1：
```
输入：s = "hello"
输出："holle"
```

##### 示例 2：
```
输入：s = "leetcode"
输出："leotcede"
```

##### 提示：
- 1 <= s.length <= 3 * 10<sup>5</sup>
- s 由 可打印的 ASCII 字符组成

##### 题解：
```rust
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut ss = s.chars().collect::<Vec<_>>();
        let L = ss.len();
        let mut i = 0;
        let mut j = L - 1;

        while i < j && j < L {
            if Self::is_vowels(ss[i]) && Self::is_vowels(ss[j]) {
                ss.swap(i, j);
                i += 1;
                j -= 1;
                continue;
            }
            if !Self::is_vowels(ss[i]) {
                i += 1;
            }
            if !Self::is_vowels(ss[j]) {
                j -= 1;
            }
        }

        ss.into_iter().collect::<String>()
    }

    fn is_vowels(c: char) -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => true,
            'A' | 'E' | 'I' | 'O' | 'U' => true,
            _ => false
        }
    }
}
```
