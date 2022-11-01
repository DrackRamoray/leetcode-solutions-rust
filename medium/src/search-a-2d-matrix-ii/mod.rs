pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let n = matrix.len() - 1;
        let m = matrix[0].len() - 1;

        let mut i = 0;
        let mut j = m;

        while i <= n && j <= m {
            if matrix[i][j] == target {
                return true;
            }

            if matrix[i][j] > target {
                j -= 1;
            } else {
                i += 1;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::search_matrix(vec![vec![1,4,7,11,15],vec![2,5,8,12,19],vec![3,6,9,16,22],vec![10,13,14,17,24],vec![18,21,23,26,30]], 5), true);
        assert_eq!(Solution::search_matrix(vec![vec![1,4,7,11,15],vec![2,5,8,12,19],vec![3,6,9,16,22],vec![10,13,14,17,24],vec![18,21,23,26,30]], 20), false);
    }
}
