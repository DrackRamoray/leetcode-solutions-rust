### [20. 有效的括号](https://leetcode.cn/problems/valid-parentheses/)

给定一个只包括 '('，')'，'{'，'}'，'['，']' 的字符串 s ，判断字符串是否有效。

有效字符串需满足：

1. 左括号必须用相同类型的右括号闭合。

2. 左括号必须以正确的顺序闭合。

3. 每个右括号都有一个对应的相同类型的左括号。


##### 示例 1：
```
输入：s = "()"
输出：true
```

##### 示例 2：
```
输入：s = "()[]{}"
输出：true
```

##### 示例 3：
```
输入：s = "(]"
输出：false
```

##### 题解：
```rust
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for u in s.bytes() {
            match u {
                c @ (b'(' | b'[' | b'{') => {
                    stack.push(c);
                },
                b')' if stack.len() > 0 && stack[stack.len() - 1] == b'(' => {
                    stack.pop();
                },
                b']' if stack.len() > 0 && stack[stack.len() - 1] == b'[' => {
                    stack.pop();
                },
                b'}' if stack.len() > 0 && stack[stack.len() - 1] == b'{' => {
                    stack.pop();
                },
                _ => {
                    return false;
                },
            }
        }

        stack.len() == 0
    }
}
```

`栈`
