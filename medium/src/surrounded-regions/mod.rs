struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let n = board.len();
        let m = board[0].len();

        for i in 0..n {
            Self::dfs(board, i, 0, n, m);
            Self::dfs(board, i , m - 1, n, m);
        }

        for j in 1..m-1 {
            Self::dfs(board, 0, j, n, m);
            Self::dfs(board, n - 1, j, n, m);
        }

        for i in 0..n {
            for j in 0..m {
                if board[i][j] == 'R' {
                    board[i][j] = 'O';
                } else if board[i][j] == 'O' {
                    board[i][j] = 'X';
                }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize, n: usize, m: usize) {
        if i < n && j < m && board[i][j] == 'O' {
            board[i][j] = 'R';
            Self::dfs(board, i-1, j, n, m);
            Self::dfs(board, i+1, j, n, m);
            Self::dfs(board, i, j-1, n, m);
            Self::dfs(board, i, j+1, n, m);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let mut board = vec![vec!['X','X','X','X'],vec!['X','O','O','X'],vec!['X','X','O','X'],vec!['X','O','X','X']];
        let ans = vec![vec!['X','X','X','X'],vec!['X','X','X','X'],vec!['X','X','X','X'],vec!['X','O','X','X']];
        Solution::solve(&mut board);
        assert_eq!(board, ans);

        let mut board = vec![vec!['X']];
        let ans = vec![vec!['X']];
        Solution::solve(&mut board);
        assert_eq!(board, ans);
    }
}
