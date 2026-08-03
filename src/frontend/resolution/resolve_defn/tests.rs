use crate::frontend::{
    lexer::lex,
    parser::{Parser, parse_defn::parse_defn},
    resolution::{
        resolve_defn,
        types::{ResolvedDefn, ResolvedExpr, ResolvedStmt, ResolvedType, ResolverCtx},
    },
};

fn resolve<'a>(ctx: &mut ResolverCtx<'a>, input: &'a str) -> ResolvedDefn {
    let mut p = Parser::new(lex(input));
    let parsed_defn = parse_defn(&mut p).unwrap();
    resolve_defn::resolve_defn(ctx, parsed_defn)
}

#[test]
fn resolves_simple_function_definition() {
    let ctx = &mut ResolverCtx::new();
    ctx.add_value("main");
    let resolved = resolve(ctx, "fn main() -> Int { return 0; }");

    match resolved {
        ResolvedDefn::Fn {
            id,
            params,
            ret_ty,
            body,
        } => {
            assert_eq!(id, ctx.lookup_value("main").unwrap());
            assert!(params.is_empty());
            assert!(matches!(ret_ty, ResolvedType::Int));
            assert_eq!(body.len(), 1);
            assert!(matches!(
                body[0],
                ResolvedStmt::Return(ResolvedExpr::Int(0))
            ));
        }
        _ => panic!("expected function definition"),
    }
}

#[test]
fn resolves_function_with_parameters_and_body() {
    let ctx = &mut ResolverCtx::new();
    ctx.add_value("add1");
    let resolved = resolve(ctx, "fn add1(x: Int) -> Int { let y = 1; return x + y; }");
    match resolved {
        ResolvedDefn::Fn {
            id: _,
            params,
            ret_ty,
            body,
        } => {
            assert_eq!(params.len(), 1);
            assert_eq!(ret_ty, ResolvedType::Int);
            assert_eq!(body.len(), 2);
            assert!(matches!(
                &body[0],
                ResolvedStmt::Let(_, ResolvedExpr::Int(1))
            ));
            assert!(
                matches!(&body[1], ResolvedStmt::Return(ResolvedExpr::Plus(left, right))
                    if matches!(&**left, ResolvedExpr::Id(_)) && matches!(&**right, ResolvedExpr::Id(_)))
            );
        }
        _ => panic!("expected function definition"),
    }
}

#[test]
fn resolves_simple_typedef_definition() {
    let ctx = &mut ResolverCtx::new();
    ctx.add_type("Option");
    ctx.add_value("some");
    ctx.add_value("none");
    let resolved = resolve(ctx, "typedef Option('a) { some(x: 'a), none() }");

    match resolved {
        ResolvedDefn::Typedef {
            id: _,
            params,
            arms,
        } => {
            assert_eq!(params.len(), 1);
            assert_eq!(arms.len(), 2);
            assert_eq!(arms[0].1.len(), 1);
            assert_eq!(arms[1].1.len(), 0);
        }
        _ => panic!("expected typedef definition"),
    }
}

#[test]
fn resolves_typedef_with_multiple_type_params_and_arms() {
    let ctx = &mut ResolverCtx::new();
    ctx.add_type("Result");
    ctx.add_value("ok");
    ctx.add_value("err");
    let resolved = resolve(
        ctx,
        "typedef Result('ok, 'err) { ok(good: 'ok), err(bad: 'err) }",
    );
    match resolved {
        ResolvedDefn::Typedef {
            id: _,
            params,
            arms,
        } => {
            assert_eq!(params.len(), 2);
            assert_eq!(arms.len(), 2);
            assert_eq!(arms[0].1.len(), 1);
            assert_eq!(arms[1].1.len(), 1);
        }
        _ => panic!("expected typedef definition"),
    }
}

// TODO
// mutually recursive types
// error case - using a generic type parameter that is not defined in the typedef
// success case - generic functions
