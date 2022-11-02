pub struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut edges = vec![vec![];num_courses as usize];
        let mut indeg = vec![0;num_courses as usize];

        for p in prerequisites.iter() {
            edges[p[1] as usize].push(p[0]);
            indeg[p[0] as usize] += 1;
        }

        let mut queue = std::collections::VecDeque::new();

        for i in 0..num_courses as usize {
            if indeg[i] == 0 {
                queue.push_back(i);
            }
        }

        let mut ans = vec![];

        while let Some(i) = queue.pop_front() {
            ans.push(i as i32);

            for &j in edges[i].iter() {
                indeg[j as usize] -= 1;

                if indeg[j as usize] == 0 {
                    queue.push_back(j as usize);
                }
            }
        }


        if ans.len() != num_courses as usize {
            vec![]
        } else {
            ans
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_order(2, vec![vec![1,0]]), vec![0,1]);
        assert_eq!(Solution::find_order(4, vec![vec![1,0],vec![2,0],vec![3,1],vec![3,2]]), vec![0,1,2,3]);
        assert_eq!(Solution::find_order(1, vec![] as Vec<Vec<i32>>), vec![0]);
    }
}
