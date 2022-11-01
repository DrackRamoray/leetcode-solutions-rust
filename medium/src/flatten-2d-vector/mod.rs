struct Vector2D {
    pc: usize,
    data: Vec<i32>,
}

impl Vector2D {

    fn new(vec: Vec<Vec<i32>>) -> Self {
        Self {
            pc: 0,
            data: vec.into_iter().flatten().collect(),
        }
    }

    fn next(&mut self) -> i32 {
        if self.pc >= self.data.len() {
            return self.data[self.data.len() - 1];
        }

        let v = self.data[self.pc];

        self.pc += 1;

        v
    }

    fn has_next(&mut self) -> bool {
        self.pc < self.data.len()
    }
}

#[test]
fn it_works() {
    let v = vec![vec![1, 2], vec![3], vec![4]];
    let mut obj = Vector2D::new(v);
    assert_eq!(obj.next(), 1);
    assert_eq!(obj.next(), 2);
    assert_eq!(obj.next(), 3);
    assert_eq!(obj.has_next(), true);
    assert_eq!(obj.has_next(), true);
    assert_eq!(obj.next(), 4);
    assert_eq!(obj.has_next(), false);
}
