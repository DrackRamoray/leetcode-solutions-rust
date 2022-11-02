pub struct Solution;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();

        for i in 1..n {
            grid[i][0] = grid[i-1][0] + grid[i][0];
        }

        for j in 1..m {
            grid[0][j] = grid[0][j-1] + grid[0][j];
        }

        for i in 1..n {
            for j in 1..m {
                grid[i][j] = (grid[i-1][j]).min(grid[i][j-1]) + grid[i][j];
            }
        }

        grid[n-1][m-1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::min_path_sum(vec![vec![1,3,1],vec![1,5,1],vec![4,2,1]]), 7);
        assert_eq!(Solution::min_path_sum(vec![vec![1,2,3],vec![4,5,6]]), 12);
    }
}
