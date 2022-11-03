### [282. 给表达式添加运算符](https://leetcode.cn/problems/expression-add-operators/)
给定一个仅包含数字 0-9 的字符串 num 和一个目标值整数 target ，在 num 的数字之间添加 二元 运算符（不是一元）+、- 或 * ，返回 所有 能够得到 target 的表达式。

注意，返回表达式中的操作数 不应该 包含前导零。



##### 示例 1:
```
输入: num = "123", target = 6
输出: ["1+2+3", "1*2*3"]
解释: “1*2*3” 和 “1+2+3” 的值都是6。
```

##### 示例 2:
```
输入: num = "232", target = 8
输出: ["2*3+2", "2+3*2"]
解释: “2*3+2” 和 “2+3*2” 的值都是8。
```

##### 示例 3:
```
输入: num = "3456237490", target = 9191
输出: []
解释: 表达式 “3456237490” 无法得到 9191 。
```

##### 提示：
- 1 <= num.length <= 10
- num 仅含数字
- -2<sup>31</sup> <= target <= 2<sup>31</sup> - 1

##### 题解：
```rust
impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        let mut ans = vec![];

        Self::dfs(&num, &mut ans, "".to_owned(), 0, target as i64, 0, 0);

        ans
    }

    fn dfs(s: &str, ans: &mut Vec<String>, selected: String, index: usize, target: i64, prev: i64, curr: i64) {
        if index == s.len() {
            if curr == target {
                ans.push(selected);
            }
            return;
        }

        let ss = s.as_bytes();

        for i in index..s.len() {
            if i != index && ss[index] == b'0' {
                break;
            }

            let val = (&s[index..i+1]).parse::<i64>().unwrap();

            if index == 0 {
                Self::dfs(&s, ans, (&s[index..i+1]).to_string(), i + 1, target, val, val);
            } else {
                Self::dfs(&s, ans, selected.to_owned() + "+" + &s[index..i+1], i + 1, target, val, curr + val);
                Self::dfs(&s, ans, selected.to_owned() + "-" + &s[index..i+1], i + 1, target, -val, curr - val);
                Self::dfs(&s, ans, selected.to_owned() + "*" + &s[index..i+1], i + 1, target, prev * val, curr - prev + prev * val);
            }
        }
    }
}
```
