struct UnionFind {
    parents: Vec<usize>,
    weights: Vec<f64>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parents = vec![0;n];
        let mut weights = vec![0.0;n];

        for i in 0..n {
            parents[i] = i;
            weights[i] = 1.0;
        }

        Self {
            parents,
            weights,
        }
    }

    fn uion(&mut self, i: usize, j: usize, weight: f64) {
        let x = self.find(i);
        let y = self.find(j);

        if x == y {
            return;
        }

        self.parents[x] = y;
        self.weights[x] = self.weights[j] * weight / self.weights[i];
    }

    fn find(&mut self, i: usize) -> usize {
        if i != self.parents[i] {
            let x = self.parents[i];
            self.parents[i] = self.find(self.parents[i]);
            self.weights[i] *= self.weights[x];
        }

        return self.parents[i];
    }

    fn is_connected(&mut self, i: usize, j: usize) -> f64 {
        let x = self.find(i);
        let y = self.find(j);

        if x == y {
            self.weights[i] / self.weights[j]
        } else {
            -1.0
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn calc_equation(equations: Vec<Vec<String>>, values: Vec<f64>, queries: Vec<Vec<String>>) -> Vec<f64> {
        let n = equations.len();
        let mut un = UnionFind::new(2 * n);
        let mut mp = std::collections::HashMap::new();
        let mut id = 0;

        for i in 0..n {
            let equation = &equations[i];
            let v1 = &equation[0];
            let v2 = &equation[1];

            if !mp.contains_key(&v1) {
                mp.insert(v1, id);
                id += 1;
            }

            if !mp.contains_key(&v2) {
                mp.insert(v2, id);
                id += 1;
            }

            if let (Some(&u), Some(&v)) = (mp.get(&v1), mp.get(&v2)) {
                un.uion(u, v, values[i]);
            }
        }

        let m = queries.len();
        let mut ans = vec![-1.0;m];

        for i in 0..m {
            let query = &queries[i];
            let v1 = &query[0];
            let v2 = &query[1];

            if let (Some(&u), Some(&v)) = (mp.get(&v1), mp.get(&v2)) {
                ans[i] = un.is_connected(u, v);
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    use assist::vec_stringify;

    let equations = vec![vec_stringify!(["a", "b"]), vec_stringify!(["b","c"])];
    let values = vec![2.0, 3.0];
    let queries = vec![vec_stringify!(["a","c"]),vec_stringify!(["b","a"]),vec_stringify!(["a","e"]),vec_stringify!(["a","a"]),vec_stringify!(["x","x"])];
    assert_eq!(Solution::calc_equation(equations, values, queries), vec![6.00000,0.50000,-1.00000,1.00000,-1.00000]);

    let equations = vec![vec_stringify!(["a","b"]),vec_stringify!(["b","c"]),vec_stringify!(["bc","cd"])];
    let values = vec![1.5,2.5,5.0];
    let queries = vec![vec_stringify!(["a","c"]),vec_stringify!(["c","b"]),vec_stringify!(["bc","cd"]),vec_stringify!(["cd","bc"])];
    assert_eq!(Solution::calc_equation(equations, values, queries), vec![3.75000,0.40000,5.00000,0.20000]);

    let equations = vec![vec_stringify!(["a", "b"])];
    let values = vec![0.5];
    let queries = vec![vec_stringify!(["a","b"]),vec_stringify!(["b","a"]),vec_stringify!(["a","c"]),vec_stringify!(["x","y"])];
    assert_eq!(Solution::calc_equation(equations, values, queries), vec![0.50000,2.00000,-1.00000,-1.00000]);
}
