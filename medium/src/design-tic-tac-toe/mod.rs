pub struct TicTacToe {
    row_sum: [Vec<i32>;2],
    col_sum: [Vec<i32>;2],
    left_diag: [i32;2],
    right_diag: [i32;2],
    n: i32,
}

impl TicTacToe {

    fn new(n: i32) -> Self {
        Self {
            row_sum: [vec![0;n as usize], vec![0;n as usize]],
            col_sum: [vec![0;n as usize], vec![0;n as usize]],
            left_diag: [0, 0],
            right_diag: [0, 0],
            n,
        }
    }

    fn make_a_move(&mut self, row: i32, col: i32, player: i32) -> i32 {
        let row = row as usize;
        let col = col as usize;
        let id = player as usize - 1;
        self.row_sum[id][row] += 1;
        self.col_sum[id][col] += 1;

        if row == col {
            self.left_diag[id] += 1;
        }

        if row + col + 1 == self.n as usize {
            self.right_diag[id] += 1;
        }

        if self.row_sum[id][row] == self.n || self.col_sum[id][col] == self.n || self.left_diag[id] == self.n || self.right_diag[id] == self.n {
            player
        } else {
            0
        }
    }
}

#[test]
fn it_works() {
    let mut toe = TicTacToe::new(3);
    assert_eq!(toe.make_a_move(0, 0, 1), 0);
    assert_eq!(toe.make_a_move(0, 2, 2), 0);
    assert_eq!(toe.make_a_move(2, 2, 1), 0);
    assert_eq!(toe.make_a_move(1, 1, 2), 0);
    assert_eq!(toe.make_a_move(2, 0, 1), 0);
    assert_eq!(toe.make_a_move(1, 0, 2), 0);
    assert_eq!(toe.make_a_move(2, 1, 1), 1);
}
