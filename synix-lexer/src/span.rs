use crate::LineColumn;

#[derive(Debug, Clone, Default)]
pub struct Span {
    pub(crate) start: LineColumn,
    pub(crate) end: LineColumn,
}

impl Span {
    pub const fn default() -> Self {
        Self::new(LineColumn::default(), LineColumn::default())
    }

    pub const fn new(start: LineColumn, end: LineColumn) -> Self {
        Self { start, end }
    }

    pub const fn join(&self, other: &Span) -> Span {
        let start = if self.start.line < other.start.line {
            self.start
        } else {
            other.start
        };

        let end = if self.end.line > other.end.line {
            self.end
        } else {
            other.end
        };

        Span { start, end }
    }

    pub const fn start(&self) -> LineColumn {
        self.start
    }

    pub const fn end(&self) -> LineColumn {
        self.end
    }
}
