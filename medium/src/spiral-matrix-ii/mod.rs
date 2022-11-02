struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0;n as usize];n as usize];
        let mut top = 0_usize;
        let mut right = n as usize - 1;
        let mut left = 0_usize;
        let mut bottom = n as usize - 1;
        let max = n * n;
        let mut num = 1;

        if n == 1 {
            return vec![vec![1]];
        }

        while num <= max {
            for i in left..=right {
                matrix[left][i] = num;
                num += 1;
            }
            top += 1;

            for i in top..=bottom {
                matrix[i][right] = num;
                num += 1;
            }
            right -= 1;

            for i in (left..=right).rev() {
                matrix[bottom][i] = num;
                num += 1;
            }
            bottom -= 1;

            for i in (top..=bottom).rev() {
                matrix[i][left] = num;
                num += 1;
            }
            left += 1;
        }

        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::generate_matrix(3), vec![vec![1,2,3],vec![8,9,4],vec![7,6,5]]);
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
    }
}
