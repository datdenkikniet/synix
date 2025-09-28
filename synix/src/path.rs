use crate::*;

#[derive(Debug)]
pub struct LookupPath {
    pub open: Token![<],
    pub head: Ident,
    pub tail: Vec<(Token![/], Ident)>,
    pub close: Token![>],
}

impl LookupPath {
    pub fn span(&self) -> Span {
        let mut span = self.open.span.join(&self.head.span());

        for (slash, ident) in &self.tail {
            span = span.join(&slash.span).join(&ident.span());
        }

        span.join(&self.close.span)
    }
}

impl Peek for LookupPath {
    fn peek(input: &ParseBuffer) -> bool {
        <Token![<]>::peek(input)
    }
}

impl Parse for LookupPath {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let open = buffer.parse()?;
        let head = buffer.parse()?;

        let mut tail = Vec::new();
        while <Token![/]>::peek(buffer) {
            let slash = buffer.parse()?;
            let ident = buffer.parse()?;
            tail.push((slash, ident));
        }

        let close = buffer.parse()?;

        Ok(Self {
            open,
            head,
            tail,
            close,
        })
    }
}
