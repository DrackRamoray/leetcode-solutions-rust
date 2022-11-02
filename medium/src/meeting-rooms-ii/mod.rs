struct Solution;

impl Solution {
    pub fn min_meeting_rooms(intervals: Vec<Vec<i32>>) -> i32 {
        if intervals.len() == 0 {
            return 0;
        }

        let mut meetings = vec![];

        for interval in intervals.iter() {
            meetings.push((interval[0], 1));
            meetings.push((interval[1], -1));
        }

        meetings.sort();

        let mut cnt = 0;
        let mut ans = 0;

        for meeting in meetings {
            cnt += meeting.1;
            ans = ans.max(cnt);
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::min_meeting_rooms(vec![vec![0,30],vec![5,10],vec![15,20]]), 2);
    assert_eq!(Solution::min_meeting_rooms(vec![vec![7,10],vec![2,4]]), 1);
}
