### [200. 岛屿数量](https://leetcode.cn/problems/number-of-islands/)

给你一个由 '1'（陆地）和 '0'（水）组成的的二维网格，请你计算网格中岛屿的数量。

岛屿总是被水包围，并且每座岛屿只能由水平方向和/或竖直方向上相邻的陆地连接形成。

此外，你可以假设该网格的四条边均被水包围。



##### 示例 1：
```
输入：grid = [
["1","1","1","1","0"],
["1","1","0","1","0"],
["1","1","0","0","0"],
["0","0","0","0","0"]
]
输出：1
```

##### 示例 2：
```
输入：grid = [
["1","1","0","0","0"],
["1","1","0","0","0"],
["0","0","1","0","0"],
["0","0","0","1","1"]
]
输出：3
```

##### 提示：
- m == grid.length
- n == grid[i].length
- 1 <= m, n <= 300
- grid[i][j] 的值为 '0' 或 '1'

##### 题解：
```rust
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut ans = 0;

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '1' {
                    ans += 1;
                    Self::search(&mut grid, i, j);
                }
            }
        }

        ans
    }

    fn search (grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        let n = grid.len();
        let m = grid[0].len();

        grid[i][j] = '0';

        if i > 0 && grid[i-1][j] == '1' {
            Self::search(grid, i - 1, j);
        }

        if i < n - 1 && grid[i+1][j] == '1' {
            Self::search(grid, i + 1, j);
        } 

        if j > 0 && grid[i][j-1] == '1' {
            Self::search(grid, i, j - 1);
        }

        if j < m - 1 && grid[i][j+1] == '1' {
            Self::search(grid, i, j + 1);
        }
    }
}
```
