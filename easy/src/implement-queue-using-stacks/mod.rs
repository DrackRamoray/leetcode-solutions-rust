pub struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>
}

impl MyQueue {
    fn new() -> Self {
        Self{
            s1: Vec::new(),
            s2: Vec::new()
        }
    }

    fn push(&mut self, x: i32) {
        self.s1.push(x);
    }

    fn pop(&mut self) -> i32 {
        if self.s2.len() > 0 {
            return self.s2.pop().unwrap();
        }

        if self.s1.len() < 1 {
            return -1;
        }

        while self.s1.len() > 1 {
            self.s2.push(self.s1.pop().unwrap());
        }

        return self.s1.pop().unwrap();
    }

    fn peek(&self) -> i32 {
        if self.s2.len() > 0 {
            return self.s2[self.s2.len() - 1];
        }

        if self.s1.len() > 0 {
            return self.s1[0];
        }

        return -1;
    }

    fn empty(&self) -> bool {
        return self.s1.is_empty() && self.s2.is_empty();
    }
}

#[cfg(test)]
mod tests {
    use super::MyQueue;

    #[test]
    fn it_works() {
        let mut queue = MyQueue::new();
        queue.push(1);
        queue.push(2);
        assert_eq!(queue.peek(), 1);
        assert_eq!(queue.pop(), 1);
        assert_eq!(queue.empty(), false);
    }
}
