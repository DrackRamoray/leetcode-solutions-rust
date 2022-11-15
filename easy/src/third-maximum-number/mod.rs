pub struct Solution;

impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut a = i64::MIN;
        let mut b = i64::MIN;
        let mut c = i64::MIN;

        for num in nums {
            let num = num as i64;

            if num > a {
                c = b;
                b = a;
                a = num;
            } else if a > num && num > b {
                c = b;
                b = num;
            } else  if b > num && num > c {
                c = num;
            }
        }

        if c == i64::MIN {
            a as i32
        } else {
            c as i32
        }
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::third_max(vec![3,2,1]), 1);
    assert_eq!(Solution::third_max(vec![1,2]), 2);
    assert_eq!(Solution::third_max(vec![2,2,3,1]), 1);
}
