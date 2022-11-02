struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let rows = num_rows as usize;
        let mut ans = vec![];

        for i in 0..rows {
            ans.push(vec![0;i+1]);
            ans[i][0] = 1;
            ans[i][i] = 1;

            for j in 1..i {
                ans[i][j] = ans[i-1][j-1] + ans[i-1][j];
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::generate(5), vec![vec![1],vec![1,1],vec![1,2,1],vec![1,3,3,1],vec![1,4,6,4,1]]);
        assert_eq!(Solution::generate(1), vec![vec![1]]);
    }
}
