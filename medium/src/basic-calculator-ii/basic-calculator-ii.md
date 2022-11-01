### [227. 基本计算器 II](https://leetcode.cn/problems/basic-calculator-ii/)
给你一个字符串表达式 s ，请你实现一个基本计算器来计算并返回它的值。

整数除法仅保留整数部分。

你可以假设给定的表达式总是有效的。所有中间结果将在 [-2<sup>31</sup>, 2<sup>31</sup> - 1] 的范围内。

注意：不允许使用任何将字符串作为数学表达式计算的内置函数，比如 eval() 。



##### 示例 1：
```
输入：s = "3+2*2"
输出：7
```

##### 示例 2：
```
输入：s = " 3/2 "
输出：1
```

##### 示例 3：
```
输入：s = " 3+5 / 2 "
输出：5
```

##### 提示：
- 1 <= s.length <= 3 * 10<sup>5</sup>
- s 由整数和算符 ('+', '-', '*', '/') 组成，中间由一些空格隔开
- s 表示一个 有效表达式
- 表达式中的所有整数都是非负整数，且在范围 [0, 2<sup>31</sup> - 1] 内
- 题目数据保证答案是一个 32-bit 整数

##### 题解：
```rust
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let ss = s.as_bytes();
        let n = ss.len();
        let mut stack = vec![];
        let mut sign = b'+';
        let mut val = 0;

        for i in 0..n {
            if ss[i] >= b'0' && ss[i] <= b'9' {
                val = val * 10 + ss[i] as i32 - '0' as i32;
            }

            if (ss[i] != b' ' && ss[i] < b'0' || ss[i] > b'9') || i == n - 1 {
                match sign {
                    b'+' => stack.push(val),
                    b'-' => stack.push(-val),
                    b'*' => {
                        if let Some(v) = stack.pop() {
                            stack.push(v * val);
                        }
                    },
                    b'/' => {
                        if let Some(v) = stack.pop() {
                            stack.push(v / val);
                        }
                    },
                    _ => {},
                }

                sign = ss[i];
                val = 0;
            }
        }

        stack.iter().sum::<i32>()
    }
}
```
