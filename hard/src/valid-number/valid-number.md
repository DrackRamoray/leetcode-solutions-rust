### [65. 有效数字](https://leetcode.cn/problems/valid-number/)

有效数字（按顺序）可以分成以下几个部分：

1. 一个 小数 或者 整数
2. （可选）一个 'e' 或 'E' ，后面跟着一个 整数

小数（按顺序）可以分成以下几个部分：

1. 可选）一个符号字符（'+' 或 '-'）
2. 下述格式之一：
   1. 至少一位数字，后面跟着一个点 '.' 
   2. 至少一位数字，后面跟着一个点 '.' ，后面再跟着至少一位数字 
   3. 一个点 '.' ，后面跟着至少一位数字

整数（按顺序）可以分成以下几个部分：

1. （可选）一个符号字符（'+' 或 '-'）
2. 至少一位数字

部分有效数字列举如下：["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"]

部分无效数字列举如下：["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"]

给你一个字符串 s ，如果 s 是一个 有效数字 ，请返回 true 。

##### 示例 1：
```
输入：s = "0"
输出：true
```

##### 示例 2：
```
输入：s = "e"
输出：false
```

##### 示例 3：
```
输入：s = "."
输出：false
```

##### 提示：
- 1 <= s.length <= 20
- s 仅含英文字母（大写和小写），数字（0-9），加号 '+' ，减号 '-' ，或者点 '.' 。

##### 题解：
```rust
enum State {
   Begin,
   BaseInt(bool),
   BaseFloat(bool),
   Exp(bool),
   ExpVal(bool),
   Trailing(bool),
}

impl Solution {
   pub fn is_number(s: String) -> bool {
      use State::*;

      let mut state = Begin;

      for c in s.chars() {
         state = match state {
            Begin => match c {
               ' ' => Begin,
               '-'|'+' => BaseInt(false),
               '.' => BaseFloat(false),
               '0'..='9' => BaseInt(true),
               _ => return false,
            },
            BaseInt(v) => match c {
               '0'..='9' => BaseInt(true),
               '.' => BaseFloat(v),
               'e'|'E' => Exp(v),
               ' ' => Trailing(v),
               _ => return false,
            },
            BaseFloat(v) => match c {
               '0'..='9' => BaseFloat(true),
               'e'|'E' => Exp(v),
               ' ' => Trailing(v),
               _ => return false,
            },
            Exp(v) => match c {
               '0'..='9' if v => ExpVal(true),
               '-'|'+' if v  => ExpVal(false),
               _ => return false,
            },
            ExpVal(v) => match c {
               '0'..='9' => ExpVal(true),
               ' ' => Trailing(v),
               _ => return false,
            },
            Trailing(v) => match c {
               ' ' => Trailing(v),
               _ => return false,
            },
         };
      }

      match state {
         Trailing(v) | BaseInt(v) | BaseFloat(v) | ExpVal(v) => v,
         _ => false,
      }
   }
}
```

`状态机`
