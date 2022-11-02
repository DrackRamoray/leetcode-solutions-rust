struct MinStack {
    s: Vec<i32>,
    m: Vec<i32>
}

impl MinStack {

    fn new() -> Self {
        Self{
            s: Vec::new(),
            m: Vec::new()
        }
    }

    fn push(&mut self, val: i32) {
        self.s.push(val);

        if self.m.is_empty() || val < self.m[self.m.len() - 1] {
            self.m.push(val);
        } else {
            self.m.push(self.m[self.m.len() - 1]);
        }
    }

    fn pop(&mut self) {
        if self.s.is_empty() {
            return;
        }

        self.s.pop();
        self.m.pop();
    }

    fn top(&self) -> i32 {
        if self.s.is_empty() {
            return -1
        }

        self.s[self.s.len() - 1]
    }

    fn get_min(&self) -> i32 {
        if self.m.is_empty() {
            return -1
        }

        self.m[self.m.len() - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::MinStack;

    #[test]
    fn it_works() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        assert_eq!(min_stack.get_min(), -3);
        min_stack.pop();
        assert_eq!(min_stack.top(), 0);
        assert_eq!(min_stack.get_min(), -2);
    }
}
