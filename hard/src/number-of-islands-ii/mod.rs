pub struct Solution;

impl Solution {
    fn num_islands2(m: i32, n: i32, positions: Vec<Vec<i32>>) -> Vec<i32> {
        let m = m as usize;
        let n = n as usize;
        let mut uf = UnionFind::new(m * n);
        let mut grid = vec![vec![0;n];m];
        let mut group = 0;
        let mut ans = vec![];

        for position in positions {
            let r = position[0] as usize;
            let c = position[1] as usize;

            if grid[r][c] == 1 {
                ans.push(group);
                continue;
            }

            grid[r][c] = 1;
            group += 1;

            let i = r * n + c;

            if r > 0 && grid[r - 1][c] == 1 {
                let j = (r - 1) * n + c;
                if uf.union(i, j) {
                    group -= 1;
                }
            }

            if r + 1 < m && grid[r + 1][c] == 1 {
                let j = (r + 1) * n + c;
                if uf.union(i, j) {
                    group -= 1;
                }
            }

            if c > 0 && grid[r][c - 1] == 1 {
                let j = r * n + c - 1;
                if uf.union(i, j) {
                    group -= 1;
                }
            }

            if c + 1 < n && grid[r][c + 1] == 1 {
                let j = r * n + c + 1;
                if uf.union(i, j) {
                    group -= 1;
                }
            }

            ans.push(group);
        }

        ans
    }
}

struct UnionFind {
    parent: Vec<usize>,
    cnt: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();

        UnionFind {
            parent,
            cnt: n,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if i != self.parent[i] {
            self.parent[i] = self.find(self.parent[i]);
        }

        self.parent[i]
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let x = self.find(i);
        let y = self.find(j);

        if x == y {
            false
        } else {
            self.parent[x] = y;
            self.cnt -= 1;
            true
        }
    }
}

#[test]
fn it_works() {
    let m = 3;
    let n = 3;
    let positions = vec![vec![0, 0], vec![0, 1], vec![1, 2],vec! [2, 1]];
    let res = vec![1, 1, 2, 3];
    assert_eq!(Solution::num_islands2(m, n, positions), res);
}
