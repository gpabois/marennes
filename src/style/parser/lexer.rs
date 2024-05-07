#[derive(Copy, Clone)]
pub struct SourceLocation {
    line: usize,
    col: usize
}

pub struct Stream<'i> {
    pub raw: &'i str,
    pub cursor: usize,
    pub location: SourceLocation
}


