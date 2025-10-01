mod common;
use common::parse_or_pretty_err;
use synix::{
    Expr,
    lambda::{ArgAttrSet, ArgAttrSetValue, ExprLambda, LambdaArg},
    lit::ExprLit,
};

macro_rules! lambda {
    ($input:literal) => {{
        let nix = $input;

        let parsed = match parse_or_pretty_err(nix) {
            Ok(v) => v,
            Err(e) => panic!("{e}"),
        };

        let Expr::Lambda(lambda) = parsed else {
            panic!("Not a lambda");
        };

        let ExprLambda { arg, body, .. } = *lambda;

        (arg, body)
    }};
}

#[test]
pub fn unit() {
    let (arg, body) = lambda!("a: b");

    let Expr::Ident(ident) = body else {
        panic!("Body is not an ident.");
    };

    assert_eq!(ident.ident(), "b");

    let LambdaArg::Ident(ident) = arg else {
        panic!("Not an ident-lambda arg");
    };

    assert_eq!(ident.ident(), "a");
}

#[test]
pub fn basic_attrset() {
    let (arg, body) = lambda!("{ a }: b");

    let Expr::Ident(ident) = body else {
        panic!("Body is not an ident.");
    };

    assert_eq!(ident.ident(), "b");

    let LambdaArg::AttrSet(ArgAttrSet {
        binds_to,
        args,
        ellipsis,
        ..
    }) = arg
    else {
        panic!("Not an attrset-lambda arg");
    };

    assert!(binds_to.is_none());
    assert!(ellipsis.is_none());

    assert_eq!(args.len(), 1);

    let ArgAttrSetValue { ident, default } = &args[0];

    assert_eq!(ident.ident(), "a");
    assert!(default.is_none());
}

#[test]
pub fn attrset_prebind() {
    let (arg, body) = lambda!("args@{ a }: b");

    let Expr::Ident(ident) = body else {
        panic!("Body is not an ident.");
    };

    assert_eq!(ident.ident(), "b");

    let LambdaArg::AttrSet(ArgAttrSet {
        binds_to,
        args,
        ellipsis,
        ..
    }) = arg
    else {
        panic!("Not an attrset lambda-arg");
    };

    assert!(ellipsis.is_none());
    let Some(binds_to) = binds_to else {
        panic!("No binds to");
    };

    assert_eq!(binds_to.ident(), "args");

    assert_eq!(args.len(), 1);

    let ArgAttrSetValue { ident, default } = &args[0];

    assert_eq!(ident.ident(), "a");
    assert!(default.is_none());
}

#[test]
pub fn attrset_postbind() {
    let (arg, body) = lambda!("{ a }@args: b");

    let Expr::Ident(ident) = body else {
        panic!("Body is not an ident.");
    };

    assert_eq!(ident.ident(), "b");

    let LambdaArg::AttrSet(ArgAttrSet {
        binds_to,
        args,
        ellipsis,
        ..
    }) = arg
    else {
        panic!("Not an attrset lambda-arg");
    };

    assert!(ellipsis.is_none());
    let Some(binds_to) = binds_to else {
        panic!("No binds to");
    };

    assert_eq!(binds_to.ident(), "args");

    assert_eq!(args.len(), 1);

    let ArgAttrSetValue { ident, default } = &args[0];

    assert_eq!(ident.ident(), "a");
    assert!(default.is_none());
}

#[test]
pub fn ellipsis() {
    let (arg, body) = lambda!("{ a, ... }: b");

    let Expr::Ident(ident) = body else {
        panic!("Body is not an ident.");
    };

    assert_eq!(ident.ident(), "b");

    let LambdaArg::AttrSet(ArgAttrSet {
        binds_to,
        args,
        ellipsis,
        ..
    }) = arg
    else {
        panic!("Not an attrset lambda-arg");
    };

    assert!(binds_to.is_none());
    assert!(ellipsis.is_some());

    let ArgAttrSetValue { ident, default } = &args[0];

    assert_eq!(ident.ident(), "a");
    assert!(default.is_none());
}

#[test]
pub fn default() {
    let (arg, body) = lambda!("{ a ? \"hello\" }: b");

    let Expr::Ident(ident) = body else {
        panic!("Body is not an ident.");
    };

    assert_eq!(ident.ident(), "b");

    let LambdaArg::AttrSet(ArgAttrSet {
        binds_to,
        args,
        ellipsis,
        ..
    }) = arg
    else {
        panic!("Not an attrset lambda-arg");
    };

    assert!(binds_to.is_none());
    assert!(ellipsis.is_none());

    let ArgAttrSetValue { ident, default } = &args[0];

    assert_eq!(ident.ident(), "a");

    let Some(default) = default else {
        panic!("No default")
    };

    assert!(matches!(default, Expr::Lit(ExprLit::Str(str)) if str.value == "hello"))
}

#[test]
pub fn default_and_ellipsis() {
    let (arg, body) = lambda!("{ a ? \"hello\", ... }: b");

    let Expr::Ident(ident) = body else {
        panic!("Body is not an ident.");
    };

    assert_eq!(ident.ident(), "b");

    let LambdaArg::AttrSet(ArgAttrSet {
        binds_to,
        args,
        ellipsis,
        ..
    }) = arg
    else {
        panic!("Not an attrset lambda-arg");
    };

    assert!(binds_to.is_none());
    assert!(ellipsis.is_some());

    let ArgAttrSetValue { ident, default } = &args[0];

    assert_eq!(ident.ident(), "a");

    let Some(default) = default else {
        panic!("No default")
    };

    assert!(matches!(default, Expr::Lit(ExprLit::Str(str)) if str.value == "hello"))
}
