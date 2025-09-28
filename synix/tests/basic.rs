use std::{iter::repeat_n, str::FromStr};

use synix::{Expr, ParseBuffer, path::Path};
use synix_lexer::TokenStream;

fn parse_pretty_print(str: &str) -> Expr {
    let err = match synix::parse(str) {
        Ok(expr) => return expr,
        Err(e) => e,
    };

    let start = err.span().start();
    let end = err.span().end();

    println!("Start: {start:?}, end: {end:?}");

    let len = (end.column - start.column).max(1);
    assert_eq!(start.line, end.line);

    let arrows: String = repeat_n(' ', start.column)
        .chain(repeat_n('^', len))
        .collect();

    let mut lines = str.lines();

    for line in (&mut lines).take(start.line + 1) {
        println!("{line}");
    }

    println!("{arrows}");

    for leftover in lines {
        println!("{leftover}");
    }

    panic!("{err:?}");
}

#[test]
pub fn basic() {
    let nix = r#"
        { a = haha/${ "hello" }; }
    "#;

    let expr = parse_pretty_print(nix);

    panic!("{expr:#?}");
}
