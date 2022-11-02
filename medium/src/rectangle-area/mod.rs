struct Solution;

impl Solution {
    pub fn compute_area(ax1: i32, ay1: i32, ax2: i32, ay2: i32, bx1: i32, by1: i32, bx2: i32, by2: i32) -> i32 {
        let left = ax1.max(bx1);
        let right = ax2.min(bx2);
        let bottom = ay1.max(by1);
        let top = ay2.min(by2);

        let r1 = (ax2 - ax1) * (ay2 - ay1);
        let r2 = (bx2 - bx1) * (by2 - by1);
        let r3 = (right.max(left) - left) * (top.max(bottom) - bottom);

        r1 + r2 - r3
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::compute_area(-3, 0, 3, 4, 0, -1, 9, 2), 45);
        assert_eq!(Solution::compute_area(-2, -2, 2, 2, -2, -2, 2, 2), 16);
    }
}
