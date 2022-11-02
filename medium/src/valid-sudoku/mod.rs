use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row = HashSet::new();
        let mut cols = vec![HashSet::new();9];
        let mut boxes = vec![HashSet::new();9];

        for i in 0..9 {
            row.clear();

            for j in 0..9 {
                let v = board[i][j];

                if v != '.' {
                    let hash = i / 3 * 3 + j / 3;

                    if row.contains(&v) || cols[j].contains(&v) || boxes[hash].contains(&v) {
                        return false;
                    }

                    row.insert(v);
                    cols[j].insert(v);
                    boxes[hash].insert(v);
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let board = vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9'],
        ];
        assert_eq!(Solution::is_valid_sudoku(board), true);
        let board = vec![
            vec!['8','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];
        assert_eq!(Solution::is_valid_sudoku(board), false);
    }
}
