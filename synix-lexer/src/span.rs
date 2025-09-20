use crate::LineColumn;

#[derive(Debug, Clone)]
pub struct Span {
    pub start: LineColumn,
    pub end: LineColumn,
}
