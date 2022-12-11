pub struct Solution;

impl Solution {
    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by(|a, b| a[1].cmp(&b[1]));

        let mut end = points[0][1];
        let mut ans = 1;

        for point in points {
            if point[0] > end {
                end = point[1];
                ans += 1;
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::find_min_arrow_shots(vec![vec![10,16],vec![2,8],vec![1,6],vec![7,12]]), 2);
    assert_eq!(Solution::find_min_arrow_shots(vec![vec![1,2],vec![3,4],vec![5,6],vec![7,8]]), 4);
    assert_eq!(Solution::find_min_arrow_shots(vec![vec![1,2],vec![2,3],vec![3,4],vec![4,5]]), 2);
}
