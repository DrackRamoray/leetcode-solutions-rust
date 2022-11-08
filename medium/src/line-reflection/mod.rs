pub struct Solution;

impl Solution {
    pub fn is_reflected(points: Vec<Vec<i32>>) -> bool {
        let mut left = i32::MAX;
        let mut right = i32::MIN;
        let mut dup = std::collections::HashSet::new();

        for point in points.iter() {
            left = left.min(point[0]);
            right = right.max(point[0]);
            dup.insert((point[0], point[1]));
        }

        let s = left + right;

        for point in points.iter() {
            if !dup.contains(&(s-point[0], point[1])) {
                return false;
            }
        }

        true
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::is_reflected(vec![vec![1,1], vec![-1,1]]), true);
    assert_eq!(Solution::is_reflected(vec![vec![1,1], vec![-1,-1]]), false);
}
