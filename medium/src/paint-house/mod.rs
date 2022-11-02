struct Solution;

impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let len = costs.len();

        let mut r = costs[0][0];
        let mut g = costs[0][1];
        let mut b = costs[0][2];

        for i in 1..len {
            let r1 = g.min(b) + costs[i][0];
            let g1 = r.min(b) + costs[i][1];
            b = r.min(g) + costs[i][2];
            r = r1;
            g = g1;
        }

        r.min(g).min(b)
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::min_cost(vec![vec![17,2,17],vec![16,16,5],vec![14,3,19]]), 10);
    assert_eq!(Solution::min_cost(vec![vec![7,6,2]]), 2);
}
