### [224. 基本计算器](https://leetcode.cn/problems/basic-calculator/)
给你一个字符串表达式 s ，请你实现一个基本计算器来计算并返回它的值。

注意:不允许使用任何将字符串作为数学表达式计算的内置函数，比如 eval() 。



##### 示例 1：
```
输入：s = "1 + 1"
输出：2
```

##### 示例 2：
```
输入：s = " 2-1 + 2 "
输出：3
```

##### 示例 3：
```
输入：s = "(1+(4+5+2)-3)+(6+8)"
输出：23
```

##### 提示：
- 1 <= s.length <= 3 * 10<sup>5</sup>
- s 由数字、'+'、'-'、'('、')'、和 ' ' 组成
- s 表示一个有效的表达式
- '+' 不能用作一元运算(例如， "+1" 和 "+(2 + 3)" 无效)
- '-' 可以用作一元运算(即 "-1" 和 "-(2 + 3)" 是有效的)
- 输入中不存在两个连续的操作符
- 每个数字和运行的计算将适合于一个有符号的 32位 整数

##### 题解：
```rust
impl Solution {
    pub fn calculate(s: String) -> i32 {
        let ss = s.as_bytes();
        let mut ops = vec![1];
        let mut sign = 1;
        let mut ans = 0;
        let mut n = s.len();
        let mut c = 0;
        let mut i =0;

        while i < n {
            match ss[i] {
                b' ' => {
                    i += 1;
                },
                b'+' => {
                    sign = ops[c];
                    i += 1;
                },
                b'-' => {
                    sign = -ops[c];
                    i += 1;
                }
                b'(' => {
                    ops.push(sign);
                    c += 1;
                    i += 1;
                },
                b')' => {
                    ops.pop();
                    c -= 1;
                    i += 1;
                },
                _ => {
                    let mut tmp = 0;

                    while i < n && ss[i] >= b'0' && ss[i] <= b'9' {
                        tmp = tmp * 10 + ss[i] as i32 - '0' as i32;
                        i += 1;
                    }

                    ans += sign * tmp;
                }
            }
        }

        ans
    }
}
```
