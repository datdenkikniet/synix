use crate::*;

#[derive(Debug)]
pub struct ExprLet {
    pub let_: Token![let],
    pub assignments: Vec<Assignment>,
    pub in_: Token![in],
    pub body: Expr,
}

impl ExprLet {
    pub fn span(&self) -> Span {
        let assignments = self
            .assignments
            .iter()
            .fold(self.let_.span.clone(), |let_, assignment| {
                let_.join(&assignment.span())
            });

        assignments.join(&self.in_.span).join(&self.body.span())
    }
}

impl Parse for ExprLet {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let let_ = buffer.parse()?;

        let mut assignments = Vec::new();
        while !<Token![in]>::peek(buffer) {
            let assignment = buffer.parse()?;
            assignments.push(assignment);
        }

        let in_ = buffer.parse()?;
        let body = buffer.parse()?;

        Ok(Self {
            let_,
            assignments,
            in_,
            body,
        })
    }
}

impl Peek for ExprLet {
    fn peek(input: &ParseBuffer) -> bool {
        let mut input = input.fork();
        <Token![let]>::parse(&mut input).is_ok()
    }
}
