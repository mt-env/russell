use crate::frontend::parser::ast::Expr;
use crate::frontend::parser::ast::ParsedExpr;
use crate::frontend::typechecker::types::TypeValue;

pub(super) fn typecheck_expr(expr: ParsedExpr) -> Expr<TypeValue> {
    todo!()
}
