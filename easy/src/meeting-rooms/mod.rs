pub struct Solution;

impl Solution {
    pub fn can_attend_meetings(mut intervals: Vec<Vec<i32>>) -> bool {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut y = 0;

        for x in intervals.iter() {
            if y > x[0] {
                return false;
            }

            y = x[1];
        }

        true
    }
}

#[test]
fn it_works() {
    let intervals = vec![vec![0, 30], vec![5, 10], vec![15, 20]];
    assert_eq!(Solution::can_attend_meetings(intervals), false);
    let intervals = vec![vec![7, 10], vec![2, 4]];
    assert_eq!(Solution::can_attend_meetings(intervals), true);
}
