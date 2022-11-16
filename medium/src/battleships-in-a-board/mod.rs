pub struct Solution;

impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let n = board.len();
        let m = board[0].len();
        let mut ans = 0;

        for i in 0..n {
            for j in 0..m {
                if board[i][j] == 'X' {
                    if i >= 1 && board[i-1][j] == 'X' {
                        continue;
                    }
                    if j >= 1 && board[i][j-1] == 'X' {
                        continue;
                    }

                    ans += 1;
                }
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::count_battleships(vec![vec!['X','.','.','X'],vec!['.','.','.','X'],vec!['.','.','.','X']]), 2);
    assert_eq!(Solution::count_battleships(vec![vec!['.']]), 0);
}
