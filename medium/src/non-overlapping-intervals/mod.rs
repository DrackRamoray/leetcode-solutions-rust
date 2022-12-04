pub struct Solution;

impl Solution {
    pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_by(|a, b| a[1].cmp(&b[1]));

        let n = intervals.len();
        let mut tmp = intervals[0][1];
        let mut ans = 0;

        for i in 1..n {
            if intervals[i][0] < tmp {
                ans += 1;
            } else {
                tmp = intervals[i][1];
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1,2],vec![2,3],vec![3,4],vec![1,3]]), 1);
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1,2],vec![1,2],vec![1,2]]), 2);
    assert_eq!(Solution::erase_overlap_intervals(vec![vec![1,2],vec![2,3]]),0)
}
