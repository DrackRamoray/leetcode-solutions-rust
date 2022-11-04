pub struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let dirs = [0, 1, -1];
        let n = board.len();
        let m = board[0].len();

        for x in 0..n {
            for y in 0..m {
                let mut lives = 0;

                for i in 0..3 {
                    for j in 0..3 {
                        if dirs[i] == 0 && dirs[j] == 0 {
                            continue;
                        }

                        let dx = (x as i32 + dirs[i]) as usize;
                        let dy = (y as i32 + dirs[j]) as usize;

                        if dx < n && dy < m && board[dx][dy].abs() == 1 {
                            lives += 1;
                        }
                    }
                }

                if board[x][y] == 1 && (lives < 2 || lives > 3) {
                    board[x][y] = -1;
                }

                if board[x][y] == 0 && lives == 3 {
                    board[x][y] = 2;
                }
            }
        }

        for i in 0..n {
            for j in 0..m {
                if board[i][j] > 0 {
                    board[i][j] = 1;
                } else {
                    board[i][j] = 0;
                }
            }
        }
    }
}

#[test]
fn it_works() {
    let mut board = vec![vec![0,1,0],vec![0,0,1],vec![1,1,1],vec![0,0,0]];
    let ans = vec![vec![0,0,0],vec![1,0,1],vec![0,1,1],vec![0,1,0]];
    Solution::game_of_life(&mut board);
    assert_eq!(board, ans);

    let mut board = vec![vec![1,1],vec![1,0]];
    let ans = vec![vec![1,1],vec![1,1]];
    Solution::game_of_life(&mut board);
    assert_eq!(board, ans);
}
