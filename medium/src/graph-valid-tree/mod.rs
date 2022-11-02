struct Solution;

impl Solution {
    pub fn valid_tree(n: i32, edges: Vec<Vec<i32>>) -> bool {
        let n = n as usize;
        let mut un = UnionFind::new(n);

        for edge in edges {
            if !un.union(edge[0] as usize, edge[1] as usize) {
                return false;
            }
        }

        un.cnt == 1
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

    fn union(&mut self, x: usize, y: usize) -> bool {
        let a = self.find(x);
        let b = self.find(y);

        if a != b {
            self.parents[a] = b;
            self.cnt -= 1;
            true
        } else {
            false
        }
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::valid_tree(5, vec![vec![0,1],vec![0,2],vec![0,3],vec![1,4]]), true);
    assert_eq!(Solution::valid_tree(5, vec![vec![0,1],vec![1,2],vec![2,3],vec![1,3],vec![1,4]]), false);
}
