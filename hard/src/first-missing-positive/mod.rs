pub struct Solution;

impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let m = nums.len();

        nums.iter_mut().for_each(|x| if *x <= 0 { *x = m as i32 + 1 });

        for i in 0..m {
            let n = nums[i].abs() as usize;

            if n <= m {
                nums[n - 1] = -nums[n - 1].abs();
            }
        }

        for i in 0..m {
            if nums[i] > 0 {
                return i as i32 + 1;
            }
        }

        m as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::first_missing_positive(vec![1,2,0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3,4,-1,1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7,8,9,11,12]), 1);
    }
}
