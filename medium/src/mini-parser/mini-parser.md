### [385. 迷你语法分析器](https://leetcode.cn/problems/mini-parser/)
给定一个字符串 s 表示一个整数嵌套列表，实现一个解析它的语法分析器并返回解析的结果 NestedInteger 。

列表中的每个元素只可能是整数或整数嵌套列表



##### 示例 1：
```
输入：s = "324",
输出：324
解释：你应该返回一个 NestedInteger 对象，其中只包含整数值 324。
```

##### 示例 2：
```
输入：s = "[123,[456,[789]]]",
输出：[123,[456,[789]]]
解释：返回一个 NestedInteger 对象包含一个有两个元素的嵌套列表：
1. 一个 integer 包含值 123
2. 一个包含两个元素的嵌套列表：
   i.  一个 integer 包含值 456
   ii. 一个包含一个元素的嵌套列表
      a. 一个 integer 包含值 789
```

##### 提示：
- 1 <= s.length <= 5 * 10<sup>4</sup>
- s 由数字、方括号 "[]"、负号 '-' 、逗号 ','组成
- 用例保证 s 是可解析的 NestedInteger
- 输入中的所有值的范围是 [-10<sup>6</sup>, 10<sup>6</sup>]

##### 题解：
```rust
impl Solution {
    pub fn deserialize(s: String) -> NestedInteger {
        let mut ss = s.chars();
        let mut neg = 1;
        let mut num: Option<i32> = None;
        let mut stack = vec![];

        while let Some(c) = ss.next() {
            match c {
                '-' => {
                    neg = -1;
                },
                '[' => {
                    stack.push(NestedInteger::List(vec![]));
                },
                ',' | ']' => {
                    if let Some(v) = num {
                        if let Some(NestedInteger::List(ref mut items)) = stack.last_mut() {
                            items.push(NestedInteger::Int(neg * v));
                        }
                    }

                    num = None;
                    neg = 1;

                    if c == ']' && stack.len() > 1 {
                        if let Some(item) = stack.pop() {
                            if let Some(NestedInteger::List(ref mut items)) = stack.last_mut() {
                                items.push(item);
                            }
                        }
                    }
                },
                '0'..='9' => {
                    if let Some(v) = num {
                        num = Some(v * 10 + (c as i32 - '0' as i32));
                    } else {
                        num = Some(c as i32 - '0' as i32);
                    }
                },
                _ => {}
            }
        }


        if let Some(v) = stack.pop() {
            v
        } else if let Some(v) = num{
            NestedInteger::Int(neg * v)
        } else {
            NestedInteger::Int(-1000001)
        }
    }
}

```
