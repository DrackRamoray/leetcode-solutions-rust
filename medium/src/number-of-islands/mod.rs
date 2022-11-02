struct Solution;

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

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        let grid = vec![
            vec!['1','1','1','1','0'],
            vec!['1','1','0','1','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','0','0','0']
        ];
        assert_eq!(Solution::num_islands(grid), 1);

        let grid = vec![
            vec!['1','1','0','0','0'],
            vec!['1','1','0','0','0'],
            vec!['0','0','1','0','0'],
            vec!['0','0','0','1','1']
        ];
        assert_eq!(Solution::num_islands(grid), 3);
    }
}
