struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let m = n / 2;

        for i in 0..m {
            for j in 0..n {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[n-i-1][j];
                matrix[n-i-1][j] = tmp;
            }
        }

        for i in 0..n {
            for j in 0..i {
                let tmp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = tmp;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let mut matrix = vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]];
        let ans = vec![vec![7,4,1],vec![8,5,2],vec![9,6,3]];
        super::Solution::rotate(&mut matrix);
        assert_eq!(matrix, ans);
        let mut matrix = vec![vec![5,1,9,11],vec![2,4,8,10],vec![13,3,6,7],vec![15,14,12,16]];
        let ans = vec![vec![15,13,2,5],vec![14,3,4,1],vec![12,6,8,9],vec![16,7,10,11]];
        super::Solution::rotate(&mut matrix);
        assert_eq!(matrix, ans);
    }
}
