pub struct NumArray {
    s: Vec<i32>
}

impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let n = nums.len();
        let mut s = vec![0;n+1];

        for i in 0..n {
            s[i+1] = s[i] + nums[i]
        }

        NumArray{s}
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.s[(right + 1) as usize] - self.s[left as usize]
    }
}

#[test]
fn it_works() {
    let obj = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
    assert_eq!(obj.sum_range(0, 2), 1);
    assert_eq!(obj.sum_range(2, 5), -1);
    assert_eq!(obj.sum_range(0, 5), -3);
}
