pub struct Solution;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut ans = i32::MIN;
        let n = matrix.len();
        let m = matrix[0].len();

        for i in 1..=n {
            for j in 1..=m {
                let mut dp = vec![vec![0;m+1];n+1];
                dp[i][j] = matrix[i-1][j-1];
                for n in i..=n {
                    for m in j..=m {
                        dp[n][m] = dp[n-1][m] + dp[n][m-1] - dp[n-1][m-1] + matrix[n-1][m-1];

                        if dp[n][m] <= k && dp[n][m] > ans {
                            ans = dp[n][m];
                        }
                    }
                }
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::max_sum_submatrix(vec![vec![1,0,1],vec![0,-2,3]], 2), 2);
    assert_eq!(Solution::max_sum_submatrix(vec![vec![2,2,-1]],3), 3);
}
