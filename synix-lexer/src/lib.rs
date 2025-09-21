mod buffer;
mod error;
pub mod group;
mod ident;
pub mod literal;
pub mod punct;
mod span;
mod token_stream;

pub use buffer::LexBuffer;
pub use error::Error;
use group::Group;
pub use ident::Ident;
use literal::Literal;
use punct::Punct;
pub use span::Span;
pub use token_stream::TokenStream;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum TokenTree {
    Group(Group),
    Ident(Ident),
    Punct(Punct),
    Literal(Literal),
}

#[derive(Debug, Clone, Copy, Default)]
pub struct LineColumn {
    pub line: usize,
    pub column: usize,
}

pub trait Lex: Sized {
    fn lex(buffer: &mut LexBuffer) -> Result<Self>;
}

impl Lex for TokenStream {
    fn lex(input: &mut LexBuffer) -> Result<Self> {
        input.skip_ws();

        if input.peek().is_none() {
            Ok(Self::default())
        } else if Group::starts(input) {
            let group = input.lex()?;
            Ok(Self {
                trees: vec![TokenTree::Group(group)],
            })
        } else if Literal::starts(input) {
            let lit = input.lex()?;
            Ok(Self {
                trees: vec![TokenTree::Literal(lit)],
            })
        } else if Ident::starts(input.peek()) {
            let ident = input.lex()?;
            Ok(Self {
                trees: vec![TokenTree::Ident(ident)],
            })
        } else {
            todo!()
        }
    }
}
