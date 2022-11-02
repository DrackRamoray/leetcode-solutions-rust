struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut x = 0;
        let mut y = nums.len();

        while x < y {
            if nums[x] == val {
                y -= 1;
                nums[x] = nums[y];
            } else {
                x += 1;
            }
        }

        x as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::remove_element(&mut vec![3,2,2,3], 3), 2);
        assert_eq!(Solution::remove_element(&mut vec![0,1,2,2,3,0,4,2], 2), 5);
    }
}
