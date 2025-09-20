use std::iter::repeat_n;

use synix_parser::Expr;

fn parse_pretty_print(str: &str) -> Expr {
    let err = match synix_parser::parse(str) {
        Ok(expr) => return expr,
        Err(e) => e,
    };

    let start = err.span().start();
    let end = err.span().end();
    let len = end.column - start.column;
    assert_eq!(start.line, end.line);

    let arrows: String = repeat_n(' ', start.column)
        .chain(repeat_n('^', len))
        .collect();

    let mut lines = str.lines();

    for line in (&mut lines).take(start.line) {
        println!("{line}");
    }

    println!("{arrows}");

    for leftover in lines {
        println!("{leftover}");
    }

    panic!();
}

#[test]
pub fn basic() {
    let nix = r#"
        let
            x = 1;
            y = 1.232e4;
        in
            { x = x; y = y; }
    "#;

    let expr = parse_pretty_print(nix);

    panic!("{expr:#?}");
}
