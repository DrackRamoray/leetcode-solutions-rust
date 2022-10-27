pub struct Solution;

impl Solution {
    pub fn plus_one(mut digits: Vec<i32>) -> Vec<i32> {
        let n = digits.len();
        let mut carry = (digits[n-1] + 1) / 10;
        digits[n-1] = (digits[n-1] + 1) % 10;
        let mut i = n.wrapping_sub(2);

        while i < n {
            let v = (digits[i] + carry) % 10;
            carry = (digits[i] + carry) / 10;
            digits[i] = v;
            i = i.wrapping_sub(1);
        }

        if carry > 0 {
            digits.insert(0, carry);
        }

        digits
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::plus_one(vec![1,2,3]), vec![1,2,4]);
        assert_eq!(Solution::plus_one(vec![4,3,2,1]), vec![4,3,2,2]);
        assert_eq!(Solution::plus_one(vec![0]), vec![1]);
    }
}
