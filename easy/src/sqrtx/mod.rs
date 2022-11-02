pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut lo = 0;
        let mut hi = x;
        let mut ans = 0;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let s = (mid as u64) * (mid as u64);

            if s <= (x as u64) {
                ans = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(super::Solution::my_sqrt(4), 2);
        assert_eq!(super::Solution::my_sqrt(8), 2);
    }
}
