pub struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let n = gas.len();
        let mut rest = 0;
        let mut min = i32::MAX;
        let mut index = 0;

        for i in 0..n {
            rest += gas[i] - cost[i];

            if rest < min {
                min = rest;
                index = i;
            }
        }

        if rest < 0 {
            -1
        } else {
            ((index + 1) % n) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::can_complete_circuit(vec![1,2,3,4,5], vec![3,4,5,1,2]), 3);
        assert_eq!(Solution::can_complete_circuit(vec![2,3,4], vec![3,4,3]), -1);
    }
}
