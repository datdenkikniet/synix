use synix_lexer::group::Group;

use crate::Peek;
use crate::*;

#[derive(Debug)]
pub struct ExprLambda {
    pub arg: LambdaArg,
    pub colon: Token![:],
    pub body: Box<Expr>,
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

        Ok(ExprLambda {
            arg,
            colon,
            body: Box::new(body),
        })
    }
}

impl Peek for ExprLambda {
    fn peek(input: &ParseBuffer) -> bool {
        let input = &mut input.fork();
        LambdaArg::parse(input).is_ok() && <Token![:]>::parse(input).is_ok()
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
    group: Group,
    pub args: Vec<ArgAttrSetValue>,
}

impl ArgAttrSet {
    pub fn span(&self) -> Span {
        self.group.span()
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
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let mut args = Vec::new();

        let group = match buffer.next() {
            Some(TokenTree::Group(group)) if group.delimiter == Delimiter::Brace => group,
            v => {
                let span = v.map(|v| v.span()).unwrap_or(buffer.span());
                let msg = format!("Expected attribute set argument.");
                return Err(Error::new(span, msg));
            }
        };

        let buffer = &mut ParseBuffer::new(group.inner.as_ref());

        while buffer.len() != 0 {
            let ident = buffer.parse()?;

            let default = if <Token![?]>::peek(buffer) {
                let question = buffer.parse()?;
                let value = buffer.parse()?;
                Some((question, value))
            } else {
                None
            };

            let comma = if buffer.len() != 0 {
                Some(buffer.parse()?)
            } else if buffer.peek(Comma) {
                Some(buffer.parse()?)
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

        Ok(Self {
            args,
            group: group.clone(),
        })
    }
}
