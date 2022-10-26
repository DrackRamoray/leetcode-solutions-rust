### [8. 字符串转换整数 (atoi)](https://leetcode.cn/problems/string-to-integer-atoi/)

请你来实现一个 myAtoi(string s) 函数，使其能将字符串转换成一个 32 位有符号整数（类似 C/C++ 中的 atoi 函数）。

函数 myAtoi(string s) 的算法如下：

1. 读入字符串并丢弃无用的前导空格

2. 检查下一个字符（假设还未到字符末尾）为正还是负号，读取该字符（如果有）。 确定最终结果是负数还是正数。 如果两者都不存在，则假定结果为正。

3. 读入下一个字符，直到到达下一个非数字字符或到达输入的结尾。字符串的其余部分将被忽略。

4. 将前面步骤读入的这些数字转换为整数（即，"123" -> 123， "0032" -> 32）。如果没有读入数字，则整数为 0 。必要时更改符号（从步骤 2 开始）。

5. 如果整数数超过 32 位有符号整数范围 [−2<sup>31</sup>,  2<sup>31</sup> − 1] ，需要截断这个整数，使其保持在这个范围内。具体来说，小于 −2<sup>31</sup> 的整数应该被固定为 −2<sup>31</sup> ，大于 2<sup>31</sup> − 1 的整数应该被固定为 2<sup>31</sup> − 1 。
返回整数作为最终结果。

**注意：**

- 本题中的空白字符只包括空格字符 ' ' 。
- 除前导空格或数字后的其余字符串外，请勿忽略 任何其他字符。


##### 示例 1：
```
输入：s = "42"
输出：42
解释：加粗的字符串为已经读入的字符，插入符号是当前读取的字符。
第 1 步："42"（当前没有读入字符，因为没有前导空格）
         ^
第 2 步："42"（当前没有读入字符，因为这里不存在 '-' 或者 '+'）
         ^
第 3 步："42"（读入 "42"）
           ^
解析得到整数 42 。
由于 "42" 在范围 [-231, 231 - 1] 内，最终结果为 42 。
```

##### 示例 2：
```
输入：s = "   -42"
输出：-42
解释：
第 1 步："   -42"（读入前导空格，但忽视掉）
            ^
第 2 步："   -42"（读入 '-' 字符，所以结果应该是负数）
             ^
第 3 步："   -42"（读入 "42"）
               ^
解析得到整数 -42 。
由于 "-42" 在范围 [-231, 231 - 1] 内，最终结果为 -42 。
```

##### 示例 3：
```
输入：s = "4193 with words"
输出：4193
解释：
第 1 步："4193 with words"（当前没有读入字符，因为没有前导空格）
         ^
第 2 步："4193 with words"（当前没有读入字符，因为这里不存在 '-' 或者 '+'）
         ^
第 3 步："4193 with words"（读入 "4193"；由于下一个字符不是一个数字，所以读入停止）
             ^
解析得到整数 4193 。
由于 "4193" 在范围 [-231, 231 - 1] 内，最终结果为 4193 。
```

##### 提示：
- 0 <= s.length <= 200
- s 由英文字母（大写和小写）、数字（0-9）、' '、'+'、'-' 和 '.' 组成

##### 题解：
```rust
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut state = State::Start;

        for &u in s.as_bytes() {
            match state {
                State::Start => {
                    match u {
                        b' ' => {},
                        b'+' => {
                            state = State::Positive(0);
                        },
                        b'-' => {
                            state = State::Negative(0);
                        },
                        b'0'..=b'9' => {
                            state = State::Positive(u as i32 - '0' as i32);
                        },
                        _ => {
                            state = State::End(0);
                        },
                    };
                },
                State::Positive(num) => {
                    if b'0' <= u && u <= b'9' {
                        let v = u as i32 - '0' as i32;

                        if num > i32::MAX / 10 || (num == i32::MAX / 10 && v > 7)  {
                            state = State::End(i32::MAX);
                        } else {
                            state = State::Positive(num * 10 + v);
                        }
                    } else {
                        state = State::End(num);
                    }
                },
                State::Negative(num) => {
                    if b'0' <= u && u <= b'9' {
                        let v = u as i32 - '0' as i32;

                        if num < i32::MIN / 10 || (num == i32::MIN / 10 && v > 7)  {
                            state = State::End(i32::MIN);
                        } else {
                            state = State::Negative(num * 10 - v);
                        }
                    } else {
                        state = State::End(num);
                    }
                },
                State::End(_) => {
                    break;
                },
            }
        }

        match state {
            State::Start => 0,
            State::Positive(num) | State::Negative(num) | State::End(num) => num,
        }
    }
}

enum State {
    Start,
    Positive(i32),
    Negative(i32),
    End(i32),
}
```

`状态机`
