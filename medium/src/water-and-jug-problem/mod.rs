pub struct Solution;

impl Solution {
    pub fn can_measure_water(jug1_capacity: i32, jug2_capacity: i32, target_capacity: i32) -> bool {
        if jug1_capacity + jug2_capacity < target_capacity {
            return false;
        }

        if jug1_capacity == 0 || jug2_capacity == 0 {
            return target_capacity == 0 || jug1_capacity + jug2_capacity == target_capacity;
        }

        target_capacity % Self::gcd(jug1_capacity, jug2_capacity) == 0
    }

    fn gcd(x: i32, y: i32) -> i32 {
        if y == 0 {
            return x;
        }

        Self::gcd(y, x % y)
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::can_measure_water(3,5,4),true);
    assert_eq!(Solution::can_measure_water(2,6,5),false);
    assert_eq!(Solution::can_measure_water(1,2,3),true);
}
