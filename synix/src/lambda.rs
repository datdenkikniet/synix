use crate::Peek;
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
        let is_ident = Ident::peek(input);

        let input = &mut input.fork();
        let has_brace_group = {
            let has_brace = input.peek(Brace);
            let _ = input.next();
            has_brace
        };

        let followed_by_colon = <Token![:]>::parse(input).is_ok();

        (is_ident || has_brace_group) && followed_by_colon
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
        let output = if buffer.peek(Ident) {
            let ident: Ident = buffer.parse()?;
            Self::Ident(ident)
        } else if buffer.peek(Brace) {
            let set = buffer.parse()?;
            Self::AttrSet(set)
        } else {
            return Err(Error::new(buffer.span(), "Expected lambda argument"));
        };

        Ok(output)
    }
}

#[derive(Debug)]
pub struct ArgAttrSet {
    span: Span,
    pub args: Vec<ArgAttrSetValue>,
    pub ellipsis: Option<Token![...]>,
}

impl ArgAttrSet {
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Debug)]
pub struct ArgAttrSetValue {
    pub ident: Ident,
    pub default: Option<(Token![?], Expr)>,
    pub comma: Option<Token![,]>,
}

impl ArgAttrSetValue {
    pub fn span(&self) -> Span {
        let ident = self.ident.span();
        let default = self
            .default
            .as_ref()
            .map(|(q, e)| q.span.join(&e.span()))
            .unwrap_or(ident.clone());

        let comma = self
            .comma
            .as_ref()
            .map(|c| c.span.clone())
            .unwrap_or(ident.clone());

        ident.join(&default).join(&comma)
    }
}

impl Parse for ArgAttrSet {
    fn parse(input: &mut ParseBuffer) -> Result<Self> {
        let mut args = Vec::new();

        let start = input.span();

        let mut group;
        braced!(input as group else "Expected attribute set argument.");

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
                let question = group.parse()?;
                let value = group.parse()?;
                Some((question, value))
            } else {
                None
            };

            let comma = if !group.is_empty() {
                Some(group.parse()?)
            } else if group.peek(Comma) {
                Some(group.parse()?)
            } else {
                None
            };

            let arg = ArgAttrSetValue {
                ident,
                default,
                comma,
            };

            args.push(arg);
        }

        let span = start.join(&group.span());

        Ok(Self {
            args,
            ellipsis,
            span,
        })
    }
}
