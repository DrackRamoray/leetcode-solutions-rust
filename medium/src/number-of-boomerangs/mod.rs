macro_rules! dist {
    ($a:expr, $b:expr) => {
        ($a[0] - $b[0]).pow(2) + ($a[1] - $b[1]).pow(2)
    }
}

pub struct Solution;

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut mp = std::collections::HashMap::new();
        let mut ans = 0;

        for i in 0..n {
            for j in 0..n {
                if i != j {
                    let distance = dist!(points[i], points[j]);
                    *mp.entry(distance).or_insert(0) += 1;
                }
            }

            for &v in mp.values() {
                if v > 1 {
                    ans += v * (v - 1);
                }
            }

            mp.clear();
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::number_of_boomerangs(vec![vec![0,0],vec![1,0],vec![2,0]]), 2);
    assert_eq!(Solution::number_of_boomerangs(vec![vec![1,1],vec![2,2],vec![3,3]]), 2);
    assert_eq!(Solution::number_of_boomerangs(vec![vec![1,1]]), 0);
}
