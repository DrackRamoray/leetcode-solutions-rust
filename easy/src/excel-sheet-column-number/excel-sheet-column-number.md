### [171. Excel 表列序号](https://leetcode.cn/problems/excel-sheet-column-number/)
给你一个字符串 columnTitle ，表示 Excel 表格中的列名称。返回 该列名称对应的列序号 。

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

##### 示例 1:
```
输入: columnTitle = "A"
输出: 1
```

##### 示例 2:
```
输入: columnTitle = "AB"
输出: 28
```

##### 示例 3:
```
输入: columnTitle = "ZY"
输出: 701
```

##### 提示：
- 1 <= columnTitle.length <= 7
- columnTitle 仅由大写英文组成
- columnTitle 在范围 ["A", "FXSHRXW"] 内

##### 题解：
```rust
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut ans = 0_i32;

        for ch in column_title.as_bytes().iter() {
            ans = ans * 26 + *ch as i32 - 'A' as u8 as i32 + 1;
        }

        ans
    }
}
```
