use proc_macro2::LineColumn;

#[derive(Debug, Clone)]
pub struct Span {
    pub start: LineColumn,
    pub end: LineColumn,
    pub file: String,
}

impl From<proc_macro2::Span> for Span {
    fn from(value: proc_macro2::Span) -> Self {
        Self {
            start: value.start(),
            end: value.end(),
            file: value.file(),
        }
    }
}

impl Span {
    pub fn join(&self, other: &Span) -> Span {
        assert_eq!(self.file, other.file);

        let start = self.start.min(other.start);
        let end = self.end.max(other.end);

        Span {
            start,
            end,
            file: self.file.clone(),
        }
    }
}
