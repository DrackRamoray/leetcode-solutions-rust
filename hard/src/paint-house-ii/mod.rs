pub struct Solution;

impl Solution {
    pub fn min_cost_ii(costs: Vec<Vec<i32>>) -> i32 {
        let m = costs[0].len();
        let n = costs.len();
        let mut dp = vec![vec![0;m];n];

        for j in 0..m {
            dp[0][j] = costs[0][j];
        }

        for i in 1..n {
            for k in 0..m {
                let mut v = i32::MAX;
                for j in 0..m {
                    if k != j {
                        v = v.min(dp[i-1][j as usize]);
                    }
                }
                dp[i][k] = v + costs[i][k];
            }
        }

        let mut ans = i32::MAX;

        for j in 0..m {
            ans = ans.min(dp[n-1][j]);
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::min_cost_ii(vec![vec![1,5,3],vec![2,9,4]]),5);
    assert_eq!(Solution::min_cost_ii(vec![vec![1,3],vec![2,4]]), 5);
}
