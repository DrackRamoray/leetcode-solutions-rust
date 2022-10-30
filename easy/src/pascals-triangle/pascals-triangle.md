### [118. 杨辉三角](https://leetcode.cn/problems/pascals-triangle/)

给定一个非负整数 numRows，生成「杨辉三角」的前 numRows 行。

在「杨辉三角」中，每个数是它左上方和右上方的数的和。





##### 示例 1:
![img.gif](img.gif)
```
输入: numRows = 5
输出: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
```

##### 示例 2:
```
输入: numRows = 1
输出: [[1]]
```

##### 提示:
- 1 <= numRows <= 30

##### 题解：
```rust
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let rows = num_rows as usize;
        let mut ans = vec![];

        for i in 0..rows {
            ans.push(vec![0;i+1]);
            ans[i][0] = 1;
            ans[i][i] = 1;

            for j in 1..i {
                ans[i][j] = ans[i-1][j-1] + ans[i-1][j];
            }
        }

        ans
    }
}
```

`动态规划`
