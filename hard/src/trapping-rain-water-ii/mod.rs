use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let n = height_map.len();
        let m = height_map[0].len();
        let mut queue = BinaryHeap::new();
        let mut vis = vec![vec![false;m];n];

        for i in 0..n {
            for j in 0..m {
                if i == 0 || j == 0 || i == n - 1 || j == m - 1 {
                    vis[i][j] = true;
                    queue.push((Reverse(height_map[i][j]), i, j));
                }
            }
        }

        let mut ans = 0;
        let dirs = vec![(1,0),(-1,0),(0,1),(0,-1)];

        while let Some((Reverse(h), i, j)) = queue.pop() {
            for &(dx, dy) in &dirs {
                let x = (i as i32 + dx) as usize;
                let y = (j as i32 + dy) as usize;

                if x < n && y < m && !vis[x][y] {
                    vis[x][y] = true;

                    if h > height_map[x][y] {
                        ans += h - height_map[x][y];
                        queue.push((Reverse(h), x, y));
                    } else {
                        queue.push((Reverse(height_map[x][y]), x, y));
                    }
                }
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::trap_rain_water(vec![vec![1,4,3,1,3,2],vec![3,2,1,3,2,4],vec![2,3,3,2,3,1]]), 4);
    assert_eq!(Solution::trap_rain_water(vec![vec![3,3,3,3,3],vec![3,2,2,2,3],vec![3,2,1,2,3],vec![3,2,2,2,3],vec![3,3,3,3,3]]), 10);
}
