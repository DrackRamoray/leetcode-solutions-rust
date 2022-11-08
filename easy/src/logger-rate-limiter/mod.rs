use std::collections::HashMap;

pub struct Logger {
    logs: HashMap<String, i32>,
}

impl Logger {

    fn new() -> Self {
        Self {
            logs: HashMap::new(),
        }
    }

    fn should_print_message(&mut self, timestamp: i32, message: String) -> bool {
        if let Some(t) = self.logs.get(&message) {
            if timestamp - t < 10 {
                return false;
            }
        }

        self.logs.insert(message, timestamp);

        true
    }
}

#[test]
fn it_works() {
    let mut logger = Logger::new();
    assert_eq!(logger.should_print_message(1, "foo".to_string()), true);
    assert_eq!(logger.should_print_message(2, "bar".to_string()), true);
    assert_eq!(logger.should_print_message(3, "foo".to_string()), false);
    assert_eq!(logger.should_print_message(8, "bar".to_string()), false);
    assert_eq!(logger.should_print_message(10, "foo".to_string()), false);
    assert_eq!(logger.should_print_message(11, "foo".to_string()), true);
}
