use std::fmt::Display;

#[derive(Clone)]
pub struct Location {
    index: usize,
    source: String,
}

#[derive(Clone)]
pub struct Span {
    start: usize,
    end: usize,
    source: String,
}

impl Location {
    pub fn new(index: usize, source: String) -> Self {
        Self { index, source }
    }

    pub fn zero(source: String) -> Self {
        Self::new(0, source)
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn source(&self) -> String {
        self.source.clone()
    }
    
    // NOTE: zero-indexing
    pub fn line(&self) -> usize {
        let mut line = 0;   // current line
        let mut idx = 0;    // current index
        self.source.chars() 
            .for_each(|c| { 
                if idx >= self.index() { return; }  // exit upon reaching index of self.0
                idx += 1;                           // increment index
                if c == '\n' { line += 1; }         // (potentially) increment row
            }); 
        line 
    } 

    // NOTE: zero-indexing
    pub fn column(&self) -> usize {
        let mut column = 0; // current column 
        let mut idx = 0;    // current index

        self.source.chars()
            .for_each(|c| {
                if idx >= self.index() { return; }      // exit upon reachin index of self.0
                idx += 1;                               // increment index
                if c == '\n' { column = 0 }  // (potentially) increment row & reset column
                column += 1;
            });
        column - 1  // zero-indexing, for consistency
    }

    pub fn advance(&mut self) {
        self.index += 1;
    }

    pub fn advance_by(&mut self, by: usize) {
        self.index += by;
    }

    pub fn span_to(self, end: Location) -> Span {
        Span::new(self.index, end.index, self.source)
    }

    pub fn span_from(self, start: Location) -> Span {
        Span::new(start.index, self.index, self.source)
    }
}

impl Span {
    pub fn new(start: usize, end: usize, source: String) -> Self {
        Self { start, end, source }
    }

    pub fn start(&self) -> Location {
        Location::new(self.start, self.source())
    }
    
    pub fn end(&self) -> Location {
        Location::new(self.end, self.source())
    }

    pub fn source(&self) -> String {
        self.source.clone()
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line(), self.column())
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.start(), self.end())
    }
}

#[cfg(test)]
mod test {
    use super::Location;

    #[test]
    fn test_location_index_persistance() {
        let index = 14;
        let location = Location::new(index, String::new());
        assert_eq!(index, location.index());
    }

    #[test]
    fn test_location_index_arithmetic() {
        let start = 14;
        let mut location = Location::new(start, String::new());
        let expected_end = start + 6;
        location.advance_by(6);
        assert_eq!(expected_end, location.index());
    }

    #[test]
    fn test_location_line() {
        let source = String::from("hello\nworld");
        let location = Location::new(8, source);
        let expected_line = 1;  // NOTE: zero-indexing
        assert_eq!(expected_line, location.line());
    }

    #[test]
    fn test_location_column() {
        let source = String::from("hello\nworld");
        let location = Location::new(8, source);
        let expected_column = 2;    // NOTE: zero-indexing
        assert_eq!(expected_column, location.column());
    }
}
