pub struct Solution;

impl Solution {
    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let n = intervals.len();
        let mut lo = 0;
        let mut hi = n.wrapping_sub(1);
        let mut pos = n;

        while lo <= hi && hi < n {
            let mid = lo + (hi - lo) / 2;

            if intervals[mid][0] <= new_interval[0] {
                lo = mid + 1;
            } else {
                hi = mid - 1;
                pos = mid;
            }
        }

        intervals.insert(pos, new_interval);

        let mut ans = vec![intervals[0].clone()];

        for i in 1..=n {
            let j = ans.len() - 1;

            if ans[j][1] < intervals[i][0] {
                ans.push(intervals[i].clone());
            } else {
                ans[j][1] = ans[j][1].max(intervals[i][1]);
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::insert(vec![vec![1,3],vec![6,9]], vec![2,5]), vec![vec![1,5],vec![6,9]]);
        assert_eq!(Solution::insert(vec![vec![1,2],vec![3,5],vec![6,7],vec![8,10],vec![12,16]], vec![4,8]), vec![vec![1,2],vec![3,10],vec![12,16]]);
        assert_eq!(Solution::insert(vec![] as Vec<Vec<i32>>, vec![5,7]), vec![vec![5,7]]);
        assert_eq!(Solution::insert(vec![vec![1,5]], vec![2,3]), vec![vec![1,5]]);
        assert_eq!(Solution::insert(vec![vec![1,5]], vec![2,7]), vec![vec![1,7]]);
    }
}
