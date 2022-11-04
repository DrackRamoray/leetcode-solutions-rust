pub struct Solution;

impl Solution {
    pub fn min_total_distance(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut rows = vec![];
        let mut cols = vec![];

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    rows.push(i as i32);
                    cols.push(j as i32);
                }
            }
        }

        cols.sort();

        Self::min_dist(&rows, rows[rows.len() / 2]) + Self::min_dist(&cols, cols[cols.len() / 2])
    }

    fn min_dist(points: &Vec<i32>, target: i32) -> i32 {
        let mut dist = 0;

        for p in points {
            dist += (p - target).abs();
        }

        dist
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::min_total_distance(vec![vec![1,0,0,0,1],vec![0,0,0,0,0],vec![0,0,1,0,0]]), 6);
    assert_eq!(Solution::min_total_distance(vec![vec![1, 1]]), 1);
}
