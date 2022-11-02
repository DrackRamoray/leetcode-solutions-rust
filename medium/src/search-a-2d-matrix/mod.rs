struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let row = matrix.len();
        let col = matrix[0].len();
        let mut i = 0;
        let mut j = row * col - 1;

        while i <= j {
            let m = i + (j - i) / 2;
            let v = matrix[m / col][m % col];

            if v == target {
                return true;
            }

            if v < target {
                i = m + 1;
            } else {
                if m == 0 {
                    return false;
                }

                j = m - 1;
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
        assert_eq!(Solution::search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 3), true);
        assert_eq!(Solution::search_matrix(vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]], 13), false);
    }
}
