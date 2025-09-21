use std::str::FromStr;

use crate::{Error, LexBuffer, TokenTree};

#[derive(Debug, Default, Clone)]
pub struct TokenStream {
    trees: Vec<TokenTree>,
}

impl TokenStream {
    pub(crate) fn new(trees: Vec<TokenTree>) -> Self {
        Self { trees }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.trees.is_empty()
    }
}

impl AsRef<[TokenTree]> for TokenStream {
    fn as_ref(&self) -> &[TokenTree] {
        &self.trees
    }
}

#[derive(Clone)]
pub struct IntoIter {
    inner: std::vec::IntoIter<TokenTree>,
}

impl Iterator for IntoIter {
    type Item = TokenTree;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

impl IntoIterator for TokenStream {
    type Item = TokenTree;

    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            inner: self.trees.into_iter(),
        }
    }
}

impl FromStr for TokenStream {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut buffer = LexBuffer::new(s);
        buffer.lex()
    }
}
