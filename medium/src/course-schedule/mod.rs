pub struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
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

        let mut visited = 0;

        while let Some(i) = queue.pop_front() {
            visited += 1;

            for &j in edges[i].iter() {
                indeg[j as usize] -= 1;

                if indeg[j as usize] == 0 {
                    queue.push_back(j as usize);
                }
            }
        }

        visited == num_courses
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(Solution::can_finish(2, vec![vec![1,0]]), true);
        assert_eq!(Solution::can_finish(2, vec![vec![1,0],vec![0,1]]), false);
    }
}
