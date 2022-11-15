pub struct Solution;

impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        people.sort_by(|a, b| {
            if a[0] != b[0] {
                b[0].cmp(&a[0])
            } else {
                a[1].cmp(&b[1])
            }
        });

        let mut ans = vec![];

        for p in people.iter() {
            ans.insert(p[1] as usize, p.to_vec());
        }

        ans
    }
}

#[test]
fn it_works() {
    assert_eq!(Solution::reconstruct_queue(vec![vec![7,0],vec![4,4],vec![7,1],vec![5,0],vec![6,1],vec![5,2]]), vec![vec![5,0],vec![7,0],vec![5,2],vec![6,1],vec![4,4],vec![7,1]]);
    assert_eq!(Solution::reconstruct_queue(vec![vec![6,0],vec![5,0],vec![4,0],vec![3,2],vec![2,2],vec![1,4]]), vec![vec![4,0],vec![5,0],vec![2,2],vec![3,2],vec![1,4],vec![6,0]]);
}
