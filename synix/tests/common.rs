use std::fmt::Write;
use std::iter::repeat_n;

use synix::Expr;

#[allow(unused)]
pub fn parse_or_pretty_err(str: &str) -> Result<Expr, String> {
    let err = match synix::parse(str) {
        Ok(expr) => return Ok(expr),
        Err(e) => e,
    };

    let start = err.span().start();
    let end = err.span().end();

    println!("Start: {start:?}, end: {end:?}");

    let len = (end.column - start.column).max(1);
    assert_eq!(start.line, end.line);

    let mut output = String::new();
    let mut lines = str.lines();

    for line in (&mut lines).take(start.line + 1) {
        writeln!(output, "{line}").unwrap();
    }

    repeat_n(' ', start.column)
        .chain(repeat_n('^', len))
        .for_each(|v| {
            output.push(v);
        });

    output.push('\n');

    writeln!(output, "Error: {}", err.message()).unwrap();

    for leftover in lines {
        writeln!(output, "{leftover}").unwrap();
    }

    Err(output)
}
