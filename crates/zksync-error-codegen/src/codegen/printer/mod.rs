pub mod indent;

use indent::Indent;

pub struct PrettyPrinter {
    pub indentation: Indent,
    buffer: String,
}

impl PrettyPrinter {
    pub fn new(capacity: usize) -> Self {
        let buffer = String::with_capacity(capacity);
        Self {
            indentation: Default::default(),
            buffer,
        }
    }
    pub fn get_buffer(self) -> String {
        self.buffer
    }

    pub fn indent(&mut self) {
        for _ in 0..self.indentation.get_value() {
            self.buffer.push_str("   ");
        }
    }

    pub fn indent_more(&mut self) {
        self.indentation.increase();
    }
    pub fn indent_less(&mut self) {
        self.indentation.decrease();
    }

    pub fn push_str(&mut self, string: &str) {
        self.buffer.push_str(string);
    }

    pub fn push_line(&mut self, string: &str) {
        self.indent();
        self.buffer.push_str(string);
        self.buffer.push('\n');
    }

    pub fn push_block(&mut self, string: &str) {
        for line in string.lines() {
            self.push_line(line);
        }
    }

    pub fn indent_more_by(&mut self, offset: usize) {
        self.indentation.increase_by(offset)
    }

    pub fn indent_less_by(&mut self, offset: usize) {
        self.indentation.decrease_by(offset)
    }
}

const DEFAULT_CAPACITY: usize = 1024 * 1024;

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self::new(DEFAULT_CAPACITY)
    }
}
