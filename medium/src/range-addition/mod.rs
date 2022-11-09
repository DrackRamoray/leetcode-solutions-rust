pub struct Solution;

impl Solution {
    pub fn get_modified_array(length: i32, updates: Vec<Vec<i32>>) -> Vec<i32> {
        let len = length as usize;
        let mut vec = vec![0;len];

        for update in updates {
            vec[update[0] as usize] += update[2];

            if update[1] as usize + 1 < len {
                vec[update[1] as usize + 1] -= update[2];
            }
        }

        for i in 1..len {
            vec[i] += vec[i-1];
        }

        vec
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::get_modified_array(5, vec![vec![1,3,2], vec![2,4,3], vec![0,2,-2]]), vec![-2,0,3,5,3]);
}
