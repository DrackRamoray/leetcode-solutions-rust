pub struct NumMatrix {
    sum: Vec<Vec<i32>>,
}

impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut sum = vec![vec![0;m+1];n+1];
        for i in 0..n {
            for j in 0..m {
                sum[i+1][j+1] = sum[i+1][j] + sum[i][j+1] - sum[i][j] + matrix[i][j];
            }
        }

        Self {
            sum,
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let r1 = row1 as usize;
        let r2 = row2 as usize;
        let c1 = col1 as usize;
        let c2 = col2 as usize;
        let sum = &self.sum;

        sum[r2+1][c2+1] - sum[r2+1][c1] - sum[r1][c2+1] + sum[r1][c1]
    }
}

#[test]
fn it_works() {
    let matrix = vec![
        vec![3, 0, 1, 4, 2],
        vec![5, 6, 3, 2, 1],
        vec![1, 2, 0, 1, 5],
        vec![4, 1, 0, 1, 7],
        vec![1, 0, 3, 0, 5]
    ];
    let nm = NumMatrix::new(matrix);
    assert_eq!(nm.sum_region(2, 1, 4, 3), 8);
}
