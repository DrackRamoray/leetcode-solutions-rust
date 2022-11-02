use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut rows = vec![HashSet::new();9];
        let mut cols = vec![HashSet::new();9];
        let mut boxes = vec![HashSet::new();9];
        let mut spaces = vec![];

        for i in 0..9 {
            for j in 0..9 {
                let v = board[i][j];
                if v == '.' {
                    spaces.push((i, j));
                } else {
                    rows[i].insert(v);
                    cols[j].insert(v);
                    boxes[i/3*3+j/3].insert(v);
                }
            }
        }

        Self::dfs(board, &mut rows, &mut cols, &mut boxes, &spaces, 0);
    }

    fn dfs (
        board: &mut Vec<Vec<char>>,
        rows: &mut Vec<HashSet<char>>,
        cols: &mut Vec<HashSet<char>>,
        boxes: &mut Vec<HashSet<char>>,
        spaces: &Vec<(usize, usize)>,
        cur: usize
    ) -> bool {
        if cur == spaces.len() {
            return true;
        }

        let (i, j) = spaces[cur];

        for k in '1'..='9' {
            let u = i/3*3+j/3;

            if !rows[i].contains(&k) && !cols[j].contains(&k) && !boxes[u].contains(&k) {
                rows[i].insert(k);
                cols[j].insert(k);
                boxes[u].insert(k);
                board[i][j] = k;
                if Self::dfs(board, rows, cols, boxes, spaces, cur + 1) {
                    return true;
                }
                rows[i].remove(&k);
                cols[j].remove(&k);
                boxes[u].remove(&k);
            }
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let mut board = vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ];
        let ans = vec![
            vec!['5','3','4','6','7','8','9','1','2'],
            vec!['6','7','2','1','9','5','3','4','8'],
            vec!['1','9','8','3','4','2','5','6','7'],
            vec!['8','5','9','7','6','1','4','2','3'],
            vec!['4','2','6','8','5','3','7','9','1'],
            vec!['7','1','3','9','2','4','8','5','6'],
            vec!['9','6','1','5','3','7','2','8','4'],
            vec!['2','8','7','4','1','9','6','3','5'],
            vec!['3','4','5','2','8','6','1','7','9']
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(board, ans);
    }
}
