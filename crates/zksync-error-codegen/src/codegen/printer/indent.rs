#[derive(Clone, Eq, PartialEq, Default)]
pub struct Indent {
    value: usize,
}

impl Indent {
    pub fn get_value(&self) -> usize {
        self.value
    }
    pub fn increase(&mut self) {
        self.increase_by(1);
    }
    pub fn increase_by(&mut self, offset: usize) {
        self.value += offset;
    }
    pub fn decrease_by(&mut self, offset: usize) {
        self.value -= offset;
    }
    pub fn decrease(&mut self) {
        self.decrease_by(1);
    }
}
