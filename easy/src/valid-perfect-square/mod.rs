pub struct Solution;

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut x = num as f64;

        loop {
            let y = (x +  (num as f64) / x) / 2.0;

            if x - y < 1e-6 {
                break;
            }

            x = y;
        }

        return (x as i32).pow(2) == num;
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::is_perfect_square(16), true);
    assert_eq!(Solution::is_perfect_square(14), false);
}
