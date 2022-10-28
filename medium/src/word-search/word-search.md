### [79. 单词搜索](https://leetcode.cn/problems/word-search/)

80. 给定一个 m x n 二维字符网格 board 和一个字符串单词 word 。如果 word 存在于网格中，返回 true ；否则，返回 false 。

单词必须按照字母顺序，通过相邻的单元格内的字母构成，其中“相邻”单元格是那些水平相邻或垂直相邻的单元格。同一个单元格内的字母不允许被重复使用。



##### 示例 1：
```
输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
输出：true
```

##### 示例 2：
```
输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
输出：true
```

##### 示例 3：
```
输入：board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
输出：false
```

##### 提示：
- m == board.length
- n = board[i].length
- 1 <= m, n <= 6
- 1 <= word.length <= 15
- board 和 word 仅由大小写英文字母组成


##### 进阶：
- 你可以使用搜索剪枝的技术来优化解决方案，使其在 board 更大的情况下可以更快解决问题？

##### 题解：
```rust
impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let n = board.len();
        let m = board[0].len();
        let word = word.as_bytes();
        let mut visited = vec![vec![false;m];n];

        for i in 0..n {
            for j in 0..m {
                if Self::search(&board, word, &mut visited, i, j, 0) {
                    return true;
                }
            }
        }

        false
    }

    fn search (board: &Vec<Vec<char>>, word: &[u8], visited: &mut Vec<Vec<bool>>, i: usize, j: usize, index: usize) -> bool {
        if index == word.len() - 1 {
            return word[index] == board[i][j] as u8;
        }

        visited[i][j] = true;

        if board[i][j] as u8 == word[index] {
            let n = board.len();
            let m = board[0].len();

            if i > 0 && visited[i-1][j] == false && Self::search(board, word, visited, i-1, j, index+1) {
                return true;
            }

            if i < n - 1 && visited[i+1][j] == false && Self::search(board, word, visited, i+1, j, index+1) {
                return true;
            }

            if j > 0 && visited[i][j-1] == false && Self::search(board, word, visited, i, j-1, index+1) {
                return true;
            }

            if j < m - 1 && visited[i][j+1] == false && Self::search(board, word, visited, i, j+1, index+1) {
                return true;
            }
        }

        visited[i][j] = false;

        false
    }
}
```

`回溯` `矩阵`
