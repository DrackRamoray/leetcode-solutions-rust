pub struct Solution;

impl Solution {
    pub fn sort_transformed_array(nums: Vec<i32>, a: i32, b: i32, c: i32) -> Vec<i32> {
        let n = nums.len();

        if n == 0 {
            return vec![];
        }

        let fx = |x: i32| a * x * x + b * x + c;

        let mut i = 0;
        let mut j = n - 1;
        let mut index = if a >= 0 {
            n - 1
        } else {
            0
        };
        let mut ans = vec![0;n];

        while i <= j {
            let t1 = fx(nums[i]);
            let t2 = fx(nums[j]);

            if a >= 0 {
                ans[index] = if t1 >= t2 {
                    i += 1;
                    t1
                } else {
                    j -= 1;
                    t2
                };

                index = index.wrapping_sub(1); // index -= 1;
            } else {
                ans[index] = if t1 >= t2 {
                    j -= 1;
                    t2
                } else {
                    i += 1;
                    t1
                };

                index += 1;
            }
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::sort_transformed_array(vec![-4,-2,2,4], 1, 3, 5), vec![3,9,15,33]);
    assert_eq!(Solution::sort_transformed_array(vec![-4,-2,2,4], -1, 3, 5), vec![-23,-5,1,7]);
}
