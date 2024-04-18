pub struct UserGuess {
    value: i32,
}

impl UserGuess {
    pub fn new(value: i32) -> UserGuess {
        if !(1..=100).contains(&value) {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        UserGuess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
