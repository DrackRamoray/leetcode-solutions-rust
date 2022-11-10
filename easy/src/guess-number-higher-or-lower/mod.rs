pub struct Solution;

impl Solution {
    unsafe fn guess_number(n: i32) -> i32 {
        let mut lo = 1;
        let mut hi = n;

        while lo < hi {
            let mid = lo + (hi - lo) / 2;

            if guess(mid) == 0 {
                return mid
            } else if guess(mid) < 0 {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        }

        lo
    }
}
