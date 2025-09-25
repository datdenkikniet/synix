use crate::*;

#[derive(Debug)]
pub struct ExprLet {
    pub let_: Token![let],
    pub assignments: Vec<(Ident, Token![=], Expr, Token![;])>,
    pub in_: Token![in],
    pub body: Box<Expr>,
}

impl ExprLet {
    pub fn span(&self) -> Span {
        let assignments = self.assignments.iter().fold(
            self.let_.span.clone(),
            |let_, (ident, eq, expr, semi)| {
                let_.join(&ident.span())
                    .join(&eq.span)
                    .join(&expr.span())
                    .join(&semi.span)
            },
        );

        assignments.join(&self.in_.span).join(&self.body.span())
    }
}

impl Parse for ExprLet {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let let_ = buffer.parse()?;

        let mut assignments = Vec::new();
        while !<Token![in]>::peek(buffer) {
            let ident = buffer.parse()?;
            let eq = buffer.parse()?;

            let mut inner = buffer.until::<Token![;]>();
            let expr = inner.parse()?;

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
