pub struct Solution;

impl Solution {
    fn count_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(n as usize);

        for edge in edges {
            uf.union(edge[0] as usize, edge[1] as usize);
        }

        uf.cnt as i32
    }
}

struct UnionFind {
    parents: Vec<usize>,
    cnt: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            cnt: n,
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if x != self.parents[x] {
            self.parents[x] = self.find(self.parents[x]);
        }

        self.parents[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let a = self.find(x);
        let b = self.find(y);

        if a != b {
            self.parents[a] = b;
            self.cnt -= 1;
        }
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::count_components(5, vec![vec![0,1], vec![1,2], vec![3,4]]), 2);
    assert_eq!(Solution::count_components(5, vec![vec![0,1], vec![1,2], vec![2,3], vec![3,4]]), 1);
}
