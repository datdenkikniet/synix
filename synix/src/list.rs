use synix_lexer::Span;

use crate::*;

#[derive(Debug)]
pub struct ExprList {
    pub entries: Vec<ListEntry>,
    span: Span,
}

impl ExprList {
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Peek for ExprList {
    fn peek(input: &ParseBuffer) -> bool {
        input.peek(Bracket)
    }
}

impl Parse for ExprList {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let mut bracketed;
        bracketed!(buffer as bracketed else "Expected list");

        let span = bracketed.span();
        let mut entries = Vec::new();

        while !bracketed.is_empty() {
            let entry = if bracketed.peek(Ident) {
                let base: Ident = bracketed.parse()?;

                if <Token![.]>::peek(&bracketed) {
                    let access = AttributeAccess::parse_rest(Expr::Ident(base), &mut bracketed)?;
                    ListEntry::AttributeAccess(access)
                } else {
                    ListEntry::Ident(base)
                }
            } else if bracketed.peek(Paren) {
                ListEntry::Parenthesized(bracketed.parse()?)
            } else if bracketed.peek(Bracket) {
                ListEntry::List(bracketed.parse()?)
            } else if bracketed.peek(Brace) {
                ListEntry::AttrSet(bracketed.parse()?)
            } else if ExprLit::peek(&bracketed) {
                ListEntry::Lit(bracketed.parse()?)
            } else if Path::peek(&bracketed) {
                ListEntry::Path(bracketed.parse()?)
            } else {
                let msg = "Expected list entry.";
                return Err(Error::new(bracketed.span(), msg));
            };

            entries.push(entry);
        }

        Ok(Self { entries, span })
    }
}

#[derive(Debug)]
pub enum ListEntry {
    Ident(Ident),
    Parenthesized(ExprParenthesized),
    List(ExprList),
    AttrSet(ExprAttrSet),
    Lit(ExprLit),
    Path(Path),
    AttributeAccess(AttributeAccess),
}
