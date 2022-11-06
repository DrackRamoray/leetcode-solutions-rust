pub struct NumMatrix {
    m: usize,
    matrix: Vec<Vec<i32>>,
    row_sum: Vec<Vec<i32>>,
}

impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut row_sum = vec![vec![0;m];n];

        for i in 0..n {
            row_sum[i][0] = matrix[i][0];

            for j in 1..m {
                row_sum[i][j] = row_sum[i][j-1] + matrix[i][j];
            }
        }

        Self {
            m,
            row_sum,
            matrix,
        }
    }

    fn update(&mut self, row: i32, col: i32, val: i32) {
        let i = row as usize;
        let j = col as usize;
        self.matrix[i][j] = val;
        let mut k = j;

        if k == 0 {
            self.row_sum[i][j] = self.matrix[i][j];
            k += 1;
        }

        while k < self.m {
            self.row_sum[i][k] = self.row_sum[i][k-1] + self.matrix[i][k];
            k += 1;
        }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let mut ans = 0;
        let row1 = row1 as usize;
        let col1 = col1 as usize;
        let row2 = row2 as usize;
        let col2 = col2 as usize;

        for i in row1..=row2 {
            ans += if col1 == 0 {
                self.row_sum[i][col2]
            } else {
                self.row_sum[i][col2] - self.row_sum[i][col1-1]
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    let mut num_matrix = NumMatrix::new(vec![vec![3,0,1,4,2],vec![5,6,3,2,1],vec![1,2,0,1,5],vec![4,1,0,1,7],vec![1,0,3,0,5]]);
    assert_eq!(num_matrix.sum_region(2,1,4,3), 8);
    num_matrix.update(3,2,2);
    assert_eq!(num_matrix.sum_region(2,1,4,3), 10);
}
