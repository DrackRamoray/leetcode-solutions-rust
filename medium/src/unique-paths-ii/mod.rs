struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] == 1 {
            return 0;
        }

        let n = obstacle_grid.len();
        let m = obstacle_grid[0].len();

        let mut dp = vec![vec![0;m];n];

        for i in 0..n {
            if obstacle_grid[i][0] == 0 {
                dp[i][0] = 1;
            } else {
                break;
            }
        }

        for j in 0..m {
            if obstacle_grid[0][j] == 0 {
                dp[0][j] = 1;
            } else {
                break;
            }
        }

        for i in 1..n {
            for j in 1..m {
                dp[i][j] = if obstacle_grid[i][j] == 0 { dp[i-1][j] + dp[i][j-1] } else { 0 };
            }
        }

        dp[n-1][m-1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0,0,0],vec![0,1,0],vec![0,0,0]]), 2);
        assert_eq!(Solution::unique_paths_with_obstacles(vec![vec![0,1],vec![0,0]]), 1);
    }
}
