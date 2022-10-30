pub struct Solution;

impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();

        for i in 1..n {
            let m = triangle[i].len();

            for j in 0..m {
                if j == 0 {
                    triangle[i][j] += triangle[i-1][j];
                } else if j == m - 1 {
                    triangle[i][j] += triangle[i-1][j-1];
                } else {
                    triangle[i][j] += triangle[i-1][j].min(triangle[i-1][j-1]);
                }
            }
        }

        *(triangle[n-1].iter().min().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::minimum_total(vec![vec![2],vec![3,4],vec![6,5,7],vec![4,1,8,3]]), 11);
        assert_eq!(Solution::minimum_total(vec![vec![-10]]), -10);
    }
}
