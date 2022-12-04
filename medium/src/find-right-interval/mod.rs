pub struct Solution;

impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let len = intervals.len();
        let mut tmp = vec![];

        for i in 0..len {
            tmp.push((intervals[i][0], i as i32))
        }

        tmp.sort_by(|a1, a2| (a1.0).cmp(&(a2.0)));

        let mut ans = vec![0;len];

        for i in 0..len {
            let mut lo = 0;
            let mut hi = len - 1;
            let mut target = -1;

            while lo <= hi && hi < len {
                let mid = lo + (hi - lo) / 2;

                if tmp[mid].0 >= intervals[i][1] {
                    target = tmp[mid].1;
                    hi = mid - 1;
                } else {
                    lo = mid + 1;
                }
            }

            ans[i] = target;
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::find_right_interval(vec![vec![1,2]]), vec![-1]);
    assert_eq!(Solution::find_right_interval(vec![vec![3,4],vec![2,3],vec![1,2]]), vec![-1,0,1]);
    assert_eq!(Solution::find_right_interval(vec![vec![1,4],vec![2,3],vec![3,4]]), vec![-1,2,-1]);
}
