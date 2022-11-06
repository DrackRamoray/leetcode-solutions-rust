pub struct Solution;

impl Solution {
    pub fn shortest_distance(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut ans = i32::MAX;
        let mut val = 0;
        let mut sum = grid.clone();
        let mut dist = grid.clone();
        let dirs = [(0,-1),(-1,0),(0,1),(1,0)];
        let mut queue = std::collections::VecDeque::new();

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    ans = i32::MAX;
                    queue.push_back((i, j));

                    while let Some((x, y)) = queue.pop_front() {
                        for &(dx, dy) in &dirs {
                            let new_x = (x as i32 + dx) as usize;
                            let new_y = (y as i32 + dy) as usize;

                            if new_x < n && new_y < m && grid[new_x][new_y] == val {
                                grid[new_x][new_y] -= 1;
                                dist[new_x][new_y] = dist[x][y] + 1;
                                sum[new_x][new_y] += dist[new_x][new_y] - 1;
                                queue.push_back((new_x, new_y));
                                ans = ans.min(sum[new_x][new_y]);
                            }
                        }
                    }

                    val -= 1;
                }
            }
        }

        if ans == i32::MAX { -1 } else { ans }
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::shortest_distance(vec![vec![1,0,2,0,1],vec![0,0,0,0,0],vec![0,0,1,0,0]]), 7);
    assert_eq!(Solution::shortest_distance(vec![vec![1,0]]), 1);
    assert_eq!(Solution::shortest_distance(vec![vec![1]]), -1);
}
