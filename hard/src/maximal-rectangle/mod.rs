pub struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let r = matrix.len();

        if r == 0 {
            return 0;
        }

        let c = matrix[0].len();
        let mut dp = vec![vec![vec![0;2];c];r]; // 记录位置(i,j)的宽/高
        let mut ans = 0;

        for i in 0..r {
            for j in 0..c {
                if matrix[i][j] == '1' {
                    dp[i][j][0] = match j { // 记录位置(i,j)的宽
                        0 => 1,
                        _ => dp[i][j-1][0] + 1
                    };
                    dp[i][j][1] = match i { // 记录位置(i,j)的高
                        0 => 1,
                        _ => dp[i-1][j][1] + 1
                    };
                }
            }
        }

        for i in 0..r {
            for j in 0..c {
                if matrix[i][j] == '1' {
                    let max_height = dp[i][j][1]; // 位置(i,j)的高
                    let mut width = dp[i][j][0];

                    for k in 1..=max_height { // 按高度枚举
                        width = width.min(dp[i.wrapping_sub(k).wrapping_add(1)/* i-k+1 */][j][0]); // 每个高度对应的宽度
                        ans = ans.max(width * k); // 每个高度对应的面积
                    }
                }
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod  tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::maximal_rectangle(vec![vec!['1','0','1','0','0'],vec!['1','0','1','1','1'],vec!['1','1','1','1','1'],vec!['1','0','0','1','0']]), 6);
        assert_eq!(Solution::maximal_rectangle(vec![] as Vec<Vec<char>>), 0);
        assert_eq!(Solution::maximal_rectangle(vec![vec!['0']]), 0);
        assert_eq!(Solution::maximal_rectangle(vec![vec!['1']]), 1);
        assert_eq!(Solution::maximal_rectangle(vec![vec!['0','0']]), 0);
    }
}
