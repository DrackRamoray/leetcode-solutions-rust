pub struct Solution;

impl Solution {
    pub fn walls_and_gates(rooms: &mut Vec<Vec<i32>>) {
        let n = rooms.len();
        let m = rooms[0].len();

        let mut queue = std::collections::VecDeque::new();

        for i in 0..n {
            for j in 0..m {
                if rooms[i][j] == 0 {
                    queue.push_back((i, j));
                }
            }
        }

        while let Some((i, j)) = queue.pop_front() {
            if i > 0 && rooms[i-1][j] == i32::MAX {
                rooms[i-1][j] = rooms[i][j] + 1;
                queue.push_back((i-1, j));
            }
            if i < n-1 && rooms[i+1][j] == i32::MAX {
                rooms[i+1][j] = rooms[i][j] + 1;
                queue.push_back((i+1, j));
            }
            if j > 0 && rooms[i][j-1] == i32::MAX {
                rooms[i][j-1] = rooms[i][j] + 1;
                queue.push_back((i, j-1));
            }
            if j < m-1 && rooms[i][j+1] == i32::MAX {
                rooms[i][j+1] = rooms[i][j] + 1;
                queue.push_back((i, j+1));
            }
        }
    }
}

#[test]
fn it_works() {
    let mut rooms = vec![vec![2147483647,-1,0,2147483647],vec![2147483647,2147483647,2147483647,-1],vec![2147483647,-1,2147483647,-1],vec![0,-1,2147483647,2147483647]];
    let ans = vec![vec![3,-1,0,1],vec![2,2,1,-1],vec![1,-1,2,-1],vec![0,-1,3,4]];
    Solution::walls_and_gates(&mut rooms);
    assert_eq!(rooms, ans);

    let mut rooms = vec![vec![-1]];
    let ans = vec![vec![-1]];
    Solution::walls_and_gates(&mut rooms);
    assert_eq!(rooms, ans);

    let mut rooms = vec![vec![0]];
    let ans = vec![vec![0]];
    Solution::walls_and_gates(&mut rooms);
    assert_eq!(rooms, ans);

    let mut rooms = vec![vec![2147483647]];
    let ans = vec![vec![2147483647]];
    Solution::walls_and_gates(&mut rooms);
    assert_eq!(rooms, ans);
}
