use super::SourceLocation;

pub struct Stream<'i> {
    /// Source of the stream
    data: &'i str,

    /// Position in the data sequence
    cursor: isize,
    previous_rows: Vec<SourceLocation>,
    pub current_location: SourceLocation,
}

impl<'i> Stream<'i> {
    pub fn new(data: &'i str) -> Self {
        Self {
            data,
            previous_rows: vec![],
            cursor: -1,
            current_location: SourceLocation::new(1, 0),
        }
    }

    pub fn rewind(&mut self) {
        self.retreat_cursor()
    }

    /// Peek the next n-chars.
    pub fn peek<const OFFSET: isize, const SIZE: usize>(&self) -> &'i str {
        let mut start = usize::try_from(self.cursor + OFFSET).unwrap_or(0);
        let mut end = start + SIZE;

        if end > self.data.len() {
            end = self.data.len();
        }

        if start > end {
            start = end;
        }

        &self.data[start..end]
    }

    pub fn current(&self) -> Option<char> {
        self.peek::<0, 1>().chars().nth(0)
    }

    fn retreat_cursor(&mut self) {
        self.cursor -= 1;

        if self.cursor < -1 {
            self.cursor = -1;
            return;
        }

        if self.current_location.col == 0 {
            self.current_location = self.previous_rows.pop().unwrap_or_default();
        } else {
            self.current_location = self.current_location.shift_left();
        }
    }

    fn advance_cursor(&mut self) {
        let current = self.current();
        self.cursor += 1;

        if self.cursor > self.data.len() as isize {
            self.cursor = self.data.len() as isize;
            return;
        }

        if let Some('\n') = current {
            self.previous_rows.push(self.current_location);
            self.current_location = self.current_location.new_line();
        } else {
            self.current_location = self.current_location.shift_right();
        }
    }
}

impl<'i> Iterator for Stream<'i> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance_cursor();
        self.current()
    }
}
