use crate::Peek;
use crate::token::At;
use crate::*;

#[derive(Debug)]
pub struct ExprLambda {
    pub arg: LambdaArg,
    pub colon: Token![:],
    pub body: Expr,
}

impl ExprLambda {
    pub fn span(&self) -> Span {
        self.arg
            .span()
            .join(&self.colon.span)
            .join(&self.body.span())
    }
}

impl Parse for ExprLambda {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let arg: LambdaArg = buffer.parse()?;
        let colon: Token![:] = buffer.parse()?;
        let body = buffer.parse()?;

        Ok(ExprLambda { arg, colon, body })
    }
}

impl Peek for ExprLambda {
    fn peek(input: &ParseBuffer) -> bool {
        let input = &mut input.fork();
        let has_lambda_arg = LambdaArg::parse(input).is_ok();

        let followed_by_colon = <Token![:]>::parse(input).is_ok();

        has_lambda_arg && followed_by_colon
    }
}

#[derive(Debug)]
pub enum LambdaArg {
    Ident(Ident),
    AttrSet(ArgAttrSet),
}

impl LambdaArg {
    pub fn span(&self) -> Span {
        match self {
            LambdaArg::Ident(ident) => ident.span(),
            LambdaArg::AttrSet(arg_attr_set) => arg_attr_set.span(),
        }
    }
}

impl Parse for LambdaArg {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let output = if buffer.peek(Ident) && !buffer.peek_n(1, At) {
            let ident: Ident = buffer.parse()?;
            Self::Ident(ident)
        } else {
            let set = buffer.parse()?;
            Self::AttrSet(set)
        };

        Ok(output)
    }
}

#[derive(Debug)]
pub struct ArgAttrSet {
    pub binds_to: Option<Ident>,
    pub args: Vec<ArgAttrSetValue>,
    pub ellipsis: Option<Token![...]>,
    span: Span,
}

impl ArgAttrSet {
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Parse for ArgAttrSet {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        let mut args = Vec::new();

        let start = input.span();

        let mut group;
        let binds_to = if input.peek_n(1, At) {
            if input.peek(Ident) {
                let binds_to = input.parse()?;
                let _at: At = input.parse()?;
                braced!(input as group else "Expected attribute set argument.");
                Some(binds_to)
            } else {
                braced!(input as group else "Expected attribute set argument.");
                let _at: At = input.parse()?;
                let binds_to = input.parse()?;
                Some(binds_to)
            }
        } else {
            braced!(input as group else "Expected attribute set argument.");
            None
        };

        let mut ellipsis = None;
        while !group.is_empty() {
            if <Token![...]>::peek(&group) {
                ellipsis = Some(group.parse()?);

                if <Token![,]>::peek(&group) {
                    <Token![,]>::parse(&mut group)?;
                }

                if !group.is_empty() {
                    return Err(Error::new(group.span(), "Expected end of argument."));
                }

                break;
            }

            let ident = group.parse()?;

            let default = if <Token![?]>::peek(&group) {
                let _question: Token![?] = group.parse()?;
                let value = group.parse()?;
                Some(value)
            } else {
                None
            };

            let _comma: Option<Token![,]> = if !group.is_empty() {
                Some(group.parse()?)
            } else if group.peek(Comma) {
                Some(group.parse()?)
            } else {
                None
            };

            let arg = ArgAttrSetValue { ident, default };

            args.push(arg);
        }

        let span = start.join(&group.span());

        Ok(Self {
            binds_to,
            args,
            ellipsis,
            span,
        })
    }
}

#[derive(Debug)]
pub struct ArgAttrSetValue {
    pub ident: Ident,
    pub default: Option<Expr>,
}

impl ArgAttrSetValue {
    pub fn span(&self) -> Span {
        let ident = self.ident.span();
        let default = self
            .default
            .as_ref()
            .map(|v| v.span())
            .unwrap_or(ident.clone());
        ident.join(&default)
    }
}
