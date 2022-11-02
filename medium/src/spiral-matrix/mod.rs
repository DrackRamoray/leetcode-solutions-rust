struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut top = 0;
        let mut right = m - 1;
        let mut left = 0;
        let mut bottom = n - 1;
        let mut ans = vec![];

        for _ in 0..n*m {
            if left > right || right > m {
                break;
            }

            for j in left..=right {
                ans.push(matrix[top][j]);
            }
            top += 1;

            if top > bottom || bottom > n {
                break;
            }

            for i in top..=bottom {
                ans.push(matrix[i][right]);
            }
            right -= 1;

            if left > right || right > m {
                break;
            }

            for j in (left..=right).rev() {
                ans.push(matrix[bottom][j]);
            }
            bottom -= 1;

            if top > bottom || bottom > n {
                break;
            }

            for i in (top..=bottom).rev() {
                ans.push(matrix[i][left]);
            }
            left += 1;
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::spiral_order(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]), vec![1,2,3,6,9,8,7,4,5]);
        assert_eq!(super::Solution::spiral_order(vec![vec![1,2,3,4],vec![5,6,7,8],vec![9,10,11,12]]), vec![1,2,3,4,8,12,11,10,9,5,6,7]);
    }
}
