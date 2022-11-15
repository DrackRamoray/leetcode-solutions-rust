pub struct Solution;

impl Solution {
    pub fn can_cross(stones: Vec<i32>) -> bool {
        let n = stones.len();
        let mut dp = vec![vec![false;n+1];n];
        dp[0][1] = true;

        for i in 1..n {
            for j in 0..i {
                let v = (stones[i] - stones[j]) as usize;

                if v > n || !dp[j][v] {
                    continue;
                }

                dp[i][v] = true;

                if v >= 1 {
                    dp[i][v-1] = true;
                }

                if v <= n - 1 {
                    dp[i][v+1] = true;
                }

                if i == n - 1 {
                    return true;
                }
            }
        }

        false
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::can_cross(vec![0,1,3,5,6,8,12,17]), true);
    assert_eq!(Solution::can_cross(vec![0,1,2,3,4,8,9,11]), false);
}
