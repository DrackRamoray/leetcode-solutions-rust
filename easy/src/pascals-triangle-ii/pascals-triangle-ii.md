### [119. 杨辉三角 II](https://leetcode.cn/problems/pascals-triangle-ii/)

给定一个非负索引 rowIndex，返回「杨辉三角」的第 rowIndex 行。

在「杨辉三角」中，每个数是它左上方和右上方的数的和。





##### 示例 1:
![img.gif](img.gif)
```
输入: rowIndex = 3
输出: [1,3,3,1]
```

##### 示例 2:
```
输入: rowIndex = 0
输出: [1]
```

##### 示例 3:
```
输入: rowIndex = 1
输出: [1,1]
```

##### 提示:
- 0 <= rowIndex <= 33


##### 进阶：
- 你可以优化你的算法到 O(rowIndex) 空间复杂度吗？

##### 题解：
```rust
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut ans = vec![0;row_index + 1];
        ans[0] = 1;

        for i in 1..=row_index {
            ans[i as usize] = ((ans[i-1] as u64) * ((row_index - i + 1) as u64) / (i as u64)) as i32;
        }

        ans
    }
}
```

`动态规划`
