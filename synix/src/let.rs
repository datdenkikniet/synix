use crate::*;

#[derive(Debug)]
pub struct ExprLet {
    pub let_: Token![let],
    pub assignments: Vec<(Ident, Token![=], Expr, Token![;])>,
    pub in_: Token![in],
    pub body: Box<Expr>,
}

impl Parse for ExprLet {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let let_ = buffer.parse()?;

        let mut assignments = Vec::new();
        while !<Token![in]>::peek(buffer) {
            let ident = buffer.parse()?;
            let eq = buffer.parse()?;
            let expr = buffer.parse()?;
            let semi = buffer.parse()?;

            assignments.push((ident, eq, expr, semi));
        }

        let in_ = buffer.parse()?;
        let body = buffer.parse()?;

        Ok(Self {
            let_,
            assignments,
            in_,
            body: Box::new(body),
        })
    }
}

impl Peek for ExprLet {
    fn peek(input: &ParseBuffer) -> bool {
        let mut input = input.fork();
        <Token![let]>::parse(&mut input).is_ok()
    }
}
