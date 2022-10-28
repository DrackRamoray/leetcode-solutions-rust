pub struct Solution;

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

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "SEE".to_string()), true);
        assert_eq!(Solution::exist(vec![vec!['A','B','C','E'],vec!['S','F','C','S'],vec!['A','D','E','E']], "ABCB".to_string()), false);
    }
}
