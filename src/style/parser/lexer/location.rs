#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct SourceLocation {
    pub line: usize,
    pub col: usize,
}

impl SourceLocation {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
    pub fn new_line(&self) -> Self {
        Self {
            line: self.line + 1,
            col: 0,
        }
    }

    pub fn shift_right(&self) -> Self {
        Self {
            line: self.line,
            col: self.col + 1,
        }
    }

    pub fn shift_left(&self) -> Self {
        Self {
            line: self.line,
            col: self.col - 1,
        }
    }
}
