pub struct Solution;

impl Solution {
    pub fn multiply(mat1: Vec<Vec<i32>>, mat2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = mat1.len();
        let m = mat2[0].len();
        let r = mat1[0].len();
        let mut ans = vec![vec![0;m];n];

        for i in 0..n {
            for j in 0..r {
                if mat1[i][j] != 0 {
                    for k in 0..m {
                        ans[i][k] += mat1[i][j] * mat2[j][k];
                    }
                }
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    let mat1 = vec![vec![1,0,0],vec![-1,0,3]];
    let mat2 = vec![vec![7,0,0],vec![0,0,0],vec![0,0,1]];
    let ans = vec![vec![7,0,0],vec![-7,0,3]];
    assert_eq!(Solution::multiply(mat1, mat2), ans);

    let mat1 = vec![vec![0]];
    let mat2 = vec![vec![0]];
    let ans = vec![vec![0]];
    assert_eq!(Solution::multiply(mat1, mat2), ans);
}
