struct Solution;

impl Solution {
    pub fn calculate_minimum_hp(dungeon: Vec<Vec<i32>>) -> i32 {
        let n = dungeon.len();
        let m = dungeon[0].len();
        let mut dp = vec![vec![0; m]; n];

        for i in (0..n).rev() {
            for j in (0..m).rev() {
                if i + 1 < n && j + 1 < m {
                    dp[i][j] = 1.max(dp[i+1][j].min(dp[i][j+1]) - dungeon[i][j]);
                } else if i + 1 < n {
                    dp[i][j] = 1.max(dp[i + 1][j] - dungeon[i][j]);
                } else if j + 1 < m {
                    dp[i][j] = 1.max(dp[i][j + 1] - dungeon[i][j]);
                } else {
                    dp[i][j] = 1.max(1 - dungeon[i][j]);
                }
            }
        }

        dp[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::calculate_minimum_hp(vec![vec![-2,-3,3],vec![-5,-10,1],vec![10,30,-5]]), 7);
    }
}
