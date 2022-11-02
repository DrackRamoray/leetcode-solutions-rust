pub struct Solution;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let n = citations.len();
        let mut lo = 0;
        let mut hi = n - 1;

        while lo <= hi && hi < n {
            let mid = lo + (hi - lo) / 2;

            if citations[mid] >= (n - mid) as i32 {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }

        n as i32 - lo as i32
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::h_index(vec![0,1,3,5,6]), 3);
    assert_eq!(Solution::h_index(vec![1,2,100]), 2);
}
