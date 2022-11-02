struct Solution;

impl Solution {
    pub fn verify_preorder(preorder: Vec<i32>) -> bool {
        let mut stack = vec![];
        let mut root_val = i32::MIN;

        for &num in preorder.iter() {
            if num < root_val {
                return false;
            }

            while stack.len() > 0 && stack[stack.len() - 1] < num {
                if let Some(v) = stack.pop() {
                    root_val = v;
                }
            }

            stack.push(num);
        }

        true
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::verify_preorder(vec![5,2,1,3,6]), true);
    assert_eq!(Solution::verify_preorder(vec![5,2,6,1,3]), false);
}
