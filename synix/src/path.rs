use synix_lexer::literal::{LitFloat, LitInt};

use crate::{ident::LiteralOrInterpolatedIdent, *};

#[derive(Debug)]
pub enum Path {
    Lookup(LookupPath),
    Normal(DirPath),
}

impl Path {
    pub fn span(&self) -> Span {
        match self {
            Path::Lookup(lookup_path) => lookup_path.span(),
            Path::Normal(normal_path) => normal_path.span(),
        }
    }
}

impl Peek for Path {
    fn peek(input: &ParseBuffer) -> bool {
        LookupPath::peek(input) || DirPath::peek(input)
    }
}

impl Parse for Path {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        if LookupPath::peek(buffer) {
            let lookup = buffer.parse()?;
            Ok(Self::Lookup(lookup))
        } else {
            let path = buffer.parse()?;
            Ok(Self::Normal(path))
        }
    }
}

#[derive(Debug)]
pub struct PathPart {
    pub head: PathSubPart,
    pub tail: Vec<PathSubPart>,
}

impl PathPart {
    pub fn span(&self) -> Span {
        self.tail
            .iter()
            .fold(self.head.span(), |s, n| s.join(&n.span()))
    }
}

impl Parse for PathPart {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let head = buffer.parse()?;

        let mut tail = Vec::new();
        while let Ok(part) = buffer.parse() {
            let is_dot_and_alone =
                matches!(&part, PathSubPart::Dot(dot) if !dot.spacing.is_joint());
            tail.push(part);

            if is_dot_and_alone {
                break;
            }
        }

        Ok(Self { head, tail })
    }
}

#[derive(Debug)]
pub enum PathSubPart {
    Ident(LiteralOrInterpolatedIdent),
    LitInt(LitInt),
    LitFloat(LitFloat),
    Dot(Punct),
}

impl PathSubPart {
    pub fn span(&self) -> Span {
        match self {
            PathSubPart::Ident(v) => v.span(),
            PathSubPart::LitInt(v) => v.span(),
            PathSubPart::LitFloat(v) => v.span(),
            PathSubPart::Dot(dot) => dot.span(),
        }
    }
}

impl Parse for PathSubPart {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let result = if buffer.peek(LitInt) {
            let int = buffer
                .next()
                .and_then(|v| {
                    if let TokenTree::Literal(Literal::Int(i)) = v {
                        Some(i.clone())
                    } else {
                        None
                    }
                })
                .expect("A literal int");

            Self::LitInt(int)
        } else if buffer.peek(LitFloat) {
            let float = buffer
                .next()
                .and_then(|v| {
                    if let TokenTree::Literal(Literal::Float(i)) = v {
                        Some(i.clone())
                    } else {
                        None
                    }
                })
                .expect("A literal float");

            Self::LitFloat(float)
        } else if LiteralOrInterpolatedIdent::peek(buffer) {
            let ident = buffer.parse()?;
            Self::Ident(ident)
        } else if buffer.peek(Dot) {
            let Some(TokenTree::Punct(punct)) = buffer.next() else {
                unreachable!()
            };

            Self::Dot(punct.clone())
        } else {
            return Err(Error::new(buffer.span(), "Expected path part."));
        };

        Ok(result)
    }
}

#[derive(Debug)]
pub struct PathPrefix {
    pub kind: PathPrefixKind,
    span: Span,
}

impl PathPrefix {
    pub fn span(&self) -> Span {
        self.span.clone()
    }

    fn peek(buffer: &ParseBuffer) -> bool {
        let normal_path =
            buffer.peek(Slash) || <Token![./]>::peek(buffer) || <Token![~/]>::peek(buffer);

        let no_prefix_path = buffer.peek(Ident) && buffer.peek_n(1, Slash);

        normal_path || no_prefix_path
    }

    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let start = buffer.span();

        let kind = if buffer.peek(Slash) {
            let _ = <Token![/]>::parse(buffer)?;
            PathPrefixKind::Absolute
        } else if <Token![./]>::peek(buffer) {
            let _ = <Token![./]>::parse(buffer)?;
            PathPrefixKind::CurrentDir
        } else if <Token![~/]>::peek(buffer) {
            let _ = <Token![~/]>::parse(buffer)?;
            PathPrefixKind::HomeDir
        } else if buffer.peek(Ident) {
            PathPrefixKind::None
        } else {
            return Err(Error::new(buffer.span(), "Expected path"));
        };

        let span = start.join(&buffer.span());

        Ok(Self { kind, span })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathPrefixKind {
    /// No prefix (evaluation should be the
    /// same as [`PathPrefixKind::CurrentDir`]).
    None,
    /// `/`
    Absolute,
    /// `./`
    CurrentDir,
    /// `~/`
    HomeDir,
}

#[derive(Debug)]
pub struct DirPath {
    pub prefix: PathPrefix,
    pub head: PathPart,
    pub tail: Vec<PathPart>,
}

impl DirPath {
    pub fn span(&self) -> Span {
        self.tail
            .iter()
            .fold(self.prefix.span(), |all, ident| all.join(&ident.span()))
    }
}

impl Peek for DirPath {
    fn peek(input: &ParseBuffer) -> bool {
        PathPrefix::peek(input)
    }
}

impl Parse for DirPath {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let prefix = PathPrefix::parse(buffer)?;

        let mut tail = Vec::new();

        let head = buffer.parse()?;

        while buffer.peek(Slash) {
            let _ = <Token![/]>::parse(buffer)?;
            let part = buffer.parse()?;
            tail.push(part);
        }

        Ok(Self { prefix, head, tail })
    }
}

#[derive(Debug)]
pub struct LookupPath {
    pub head: Ident,
    pub tail: Vec<Ident>,
    span: Span,
}

impl LookupPath {
    pub fn span(&self) -> Span {
        self.span.clone()
    }
}

impl Peek for LookupPath {
    fn peek(input: &ParseBuffer) -> bool {
        <Token![<]>::peek(input)
    }
}

impl Parse for LookupPath {
    fn parse(buffer: &mut ParseBuffer) -> Result<Self> {
        let start = buffer.span();
        let _ = <Token![<]>::parse(buffer)?;
        let head = buffer.parse()?;

        let mut tail = Vec::new();

        while buffer.peek(Slash) {
            let _ = <Token![/]>::parse(buffer)?;
            let ident = buffer
                .parse()
                .map_err(|_| Error::new(buffer.span(), "Path has trailing slash"))?;
            tail.push(ident);
        }

        let _ = <Token![>]>::parse(buffer)?;
        let span = start.join(&buffer.span());

        Ok(Self { head, tail, span })
    }
}
