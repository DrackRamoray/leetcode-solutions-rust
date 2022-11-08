use std::collections::HashSet;
use std::collections::VecDeque;

pub struct SnakeGame {
    snake: VecDeque<(i32, i32)>,
    food: Vec<(i32, i32)>,
    screen: HashSet<(i32, i32)>,
    height: i32,
    width: i32,
    score: i32,
}

impl SnakeGame {
    fn new(width: i32, height: i32, food: Vec<Vec<i32>>) -> Self {
        let mut snake = VecDeque::new();
        snake.push_back((0, 0));
        let mut screen = HashSet::new();
        screen.insert((0, 0));

        SnakeGame {
            snake,
            food: food.iter().rev().map(|v| (v[0], v[1])).collect(),
            screen,
            height,
            width,
            score: 0,
        }
    }

    fn make_a_move(&mut self, direction: String) -> i32 {
        if self.score == -1 {
            return -1;
        }

        let head = self.snake.front().unwrap();
        let offset = match direction.as_str() {
            "U" => (-1, 0),
            "L" => (0, -1),
            "R" => (0, 1),
            "D" => (1, 0),
            _ => (0, 0),
        };
        let next = (head.0 + offset.0, head.1 + offset.1);
        let tail = self.snake.pop_back().unwrap();

        self.screen.remove(&tail);

        if next.0 < 0 || next.0 >= self.height || next.1 < 0 || next.1 >= self.width || self.screen.contains(&next) {
            return -1;
        }

        self.snake.push_front(next);
        self.screen.insert(next);

        if let Some(&food) = self.food.last() {
            if food == next {
                self.food.pop();
                self.score += 1;
                self.screen.insert(tail);
                self.snake.push_back(tail);
            }
        }

        self.score
    }
}

#[test]
fn it_works() {
    let mut snake = SnakeGame::new(3, 2, vec![vec![1, 2], vec![0, 1]]);
    assert_eq!(snake.make_a_move("R".to_string()), 0);
    assert_eq!(snake.make_a_move("D".to_string()), 0);
    assert_eq!(snake.make_a_move("R".to_string()), 1);
    assert_eq!(snake.make_a_move("U".to_string()), 1);
    assert_eq!(snake.make_a_move("L".to_string()), 2);
    assert_eq!(snake.make_a_move("U".to_string()), -1);
}
