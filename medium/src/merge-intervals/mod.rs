pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));

        let mut ans = vec![intervals[0].clone()];

        let n = intervals.len();

        for i in 1..n {
            let m = ans.len() - 1;
            if intervals[i][0] <= ans[m][1] {
                if intervals[i][1] > ans[m][1] {
                    ans[m][1] = intervals[i][1];
                }
            } else {
                ans.push(intervals[i].clone());
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::merge(vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]]), vec![vec![1,6],vec![8,10],vec![15,18]]);
        assert_eq!(super::Solution::merge(vec![vec![1,4],vec![4,5]]), vec![vec![1,5]]);
    }
}
