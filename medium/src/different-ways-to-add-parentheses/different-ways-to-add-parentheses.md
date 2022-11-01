### [241. 为运算表达式设计优先级](https://leetcode.cn/problems/different-ways-to-add-parentheses/)
给你一个由数字和运算符组成的字符串 expression ，按不同优先级组合数字和运算符，计算并返回所有可能组合的结果。你可以 按任意顺序 返回答案。

生成的测试用例满足其对应输出值符合 32 位整数范围，不同结果的数量不超过 104 。



##### 示例 1：
```
输入：expression = "2-1-1"
输出：[0,2]
解释：
((2-1)-1) = 0
(2-(1-1)) = 2
```

##### 示例 2：
```
输入：expression = "2*3-4*5"
输出：[-34,-14,-10,-10,10]
解释：
(2*(3-(4*5))) = -34
((2*3)-(4*5)) = -14
((2*(3-4))*5) = -10
(2*((3-4)*5)) = -10
(((2*3)-4)*5) = 10
```

##### 提示：
- 1 <= expression.length <= 20
- expression 由数字和算符 '+'、'-' 和 '*' 组成。
- 输入表达式中的所有整数值在范围 [0, 99] 

##### 提示：
```rust
impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        Self::compute(&expression)
    }

    fn compute(expression: &str) -> Vec<i32> {
        if Self::is_digit(expression) {
            let num = expression.parse::<i32>().unwrap();
            return vec![num];
        }

        let n = expression.len();
        let ss = expression.as_bytes();
        let mut ans = vec![];

        for i in 0..n {
            if ss[i] == b'+' || ss[i] == b'-' || ss[i] == b'*' {
                let left = Self::compute(&expression[0..i]);
                let right = Self::compute(&expression[i+1..]);

                for &left_num in left.iter() {
                    for &right_num in right.iter() {
                        let res = if ss[i] == b'+' {
                            left_num + right_num
                        } else if ss[i] == b'-' {
                            left_num - right_num
                        } else {
                            left_num * right_num
                        };
                        ans.push(res);
                    }
                }
            }
        }

        ans
    }

    fn is_digit(expression: &str) -> bool {
        for &u in expression.as_bytes() {
            if u < b'0' || u > b'9' {
                return false;
            }
        }

        true
    }
}
```
