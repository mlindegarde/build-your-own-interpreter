pub struct Caret<'a> {
    source: &'a str,
    pub start_car: u16,
    pub current_char: u16,
    pub current_line: u16
}

impl<'a> Caret<'a> {
    pub fn new(source: &'a str) -> Self {
        Caret {
            source,
            start_car: 0,
            current_char: 0,
            current_line: 1
        }
    }

    pub fn is_at_end_of_input(&self) -> bool {
        self.current_char >= self.source.chars().count() as u16
    }

    pub fn peek(&self) -> char {
        if self.is_at_end_of_input() {
            '\0'
        } else {
            self.source.chars().nth(self.current_char as usize).unwrap_or('\0')
        }
    }

    pub fn peek_next(&self) -> char {
        if self.current_char + 1 >= self.source.chars().count() as u16 {
            '\0'
        } else {
            self.source.chars().nth((self.current_char + 1) as usize).unwrap_or('\0')
        }
    }

    pub fn advance(&mut self) -> char {
        let value = self.source.chars().nth(self.current_char as usize).unwrap();
        self.current_char += 1;

        value
    }

    pub fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end_of_input() {
            return false;
        }

        match self.source.chars().nth(self.current_char as usize) {
            Some(value) if value == expected => {
                self.current_char += 1;
                true
            },
            Some(_) | None => false
        }
    }
}
