use synix_lexer::{Span, TokenTree, punct::Char};

use crate::{ParseBuffer, Result};

fn punct_helper<const N: usize>(
    buffer: &mut ParseBuffer,
    repr: &str,
    chars: [Char; N],
) -> Result<Span> {
    let spans = [const { Span::default() }; N];

    let mut chars = chars.into_iter();

    while let Some(next) = chars.next() {
        let punct = if let Some(TokenTree::Punct(punct)) = buffer.next() {
            punct
        } else {
            let msg = format!("Expected `{}`", repr);
            return Err(crate::Error::new(buffer.span(), msg));
        };

        if next != punct.ch {
            let msg = format!("Expected `{}`", repr);
            return Err(crate::Error::new(buffer.span(), msg));
        }

        if chars.len() != 0 && !punct.spacing.is_joint() {
            let msg = format!("Expected `{}`", repr);
            return Err(crate::Error::new(buffer.span(), msg));
        }
    }

    let span = spans.iter().fold(spans[0].clone(), |l, r| l.join(r));

    Ok(span)
}

macro_rules! punct_tokens {
    ($($ty:ident = $name:tt as [$($char:ident),*])*) => {
        $(
            #[derive(Debug, Default, Clone)]
            #[doc = concat!("`", stringify!($name), "`")]
            pub struct $ty {
                pub span: Span,
            }

            impl $ty {
                pub const fn new() -> Self {
                    Self {
                        span: Span::default(),
                    }
                }
            }

            impl crate::Parse for $ty {
                fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
                    let span = punct_helper(buffer, stringify!($name), [$(Char::$char,)*])?;

                    Ok(Self{ span })
                }
            }
        )*
    };
}

punct_tokens! {
    Semicolon = ; as [Semicolon]
    Colon = : as [Colon]
    Comma = , as [Comma]
    Dot = . as [Dot]
    At = @ as [At]
    Plus = + as [Plus]
    Minus = - as [Minus]
    Asterisk = * as [Asterisk]
    Gt = > as [Gt]
    Lt = < as [Lt]
    Equals = = as [Equals]
    Question = ? as [Question]
    Ampersand = & as [Ampersand]
    Pipe = | as [Pipe]
    Slash = / as [Slash]
    Ellipsis = ... as [Dot, Dot, Dot]
    Ge = >= as [Gt, Equals]
    Le = <= as [Lt, Equals]
    EqualsEquals = == as [Equals, Equals]
    And = && as [Ampersand, Ampersand]
    Or = || as [Pipe, Pipe]
}

/// `//`
#[derive(Debug, Default, Clone)]
pub struct Update {
    pub span: Span,
}

impl Update {
    pub const fn new() -> Self {
        Self {
            span: Span::default(),
        }
    }
}

impl crate::Parse for Update {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let span = punct_helper(buffer, "//", [Char::Slash, Char::Slash])?;
        Ok(Self { span })
    }
}
