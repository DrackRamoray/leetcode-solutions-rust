struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut once  = 0;
        let mut more = 0;

        for num in nums.iter() {
            once = !more & (once ^ num);
            more = !once & (more ^ num);
        }

        once
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::single_number(vec![2,2,3,2]), 3);
        assert_eq!(Solution::single_number(vec![0,1,0,1,0,1,99]), 99);
    }
}
