### [168. Excel表列名称](https://leetcode.cn/problems/excel-sheet-column-title/)
给你一个整数 columnNumber ，返回它在 Excel 表中相对应的列名称。

例如：
```
A -> 1
B -> 2
C -> 3
...
Z -> 26
AA -> 27
AB -> 28
...
```

##### 示例 1：
```
输入：columnNumber = 1
输出："A"
```

##### 示例 2：
```
输入：columnNumber = 28
输出："AB"
```

##### 示例 3：
```
输入：columnNumber = 701
输出："ZY"
```

##### 示例 4：
```
输入：columnNumber = 2147483647
输出："FXSHRXW"
```

##### 提示：
- 1 <= columnNumber <= 2<sup>31</sup> - 1

##### 题解：
```rust
impl Solution {
    pub fn convert_to_title(mut column_number: i32) -> String {
        let mut ans = vec![];

        while column_number > 0 {
            let x = ((column_number - 1) % 26) as u8;
            let c = (x + b'A') as char;
            ans.insert(0, c);
            column_number = (column_number - 1) / 26;
        }

        ans.iter().collect()
    }
}
```
