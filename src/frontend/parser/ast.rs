use std::fmt::Display;

use crate::frontend::{lexer::token::TokenKind, types::Spanned};

pub type SpannedExpr<'src, A> = Spanned<Expr<'src, A>>;
pub type SpannedStmt<'src, A> = Spanned<Stmt<'src, A>>;
pub type SpannedDefn<'src, A> = Spanned<Defn<'src, A>>;
pub type SpannedBinding<'src> = Spanned<Binding<'src>>;
pub type SpannedMatchArm<'src, A> = Spanned<MatchArm<'src, A>>;

pub type ParsedExpr<'src> = SpannedExpr<'src, ()>;
pub type ParsedStmt<'src> = SpannedStmt<'src, ()>;
pub type ParsedDefn<'src> = SpannedDefn<'src, ()>;
pub type ParsedBinding<'src> = SpannedBinding<'src>;
pub type ParsedMatchArm<'src> = SpannedMatchArm<'src, ()>;
pub type ParsedType<'src> = Spanned<Type<'src>>;
pub type ParsedConstructorArm<'src> = Spanned<ConstructorArm<'src>>;

#[derive(Debug, PartialEq)]
pub enum Defn<'src, A> {
    // typedef <typeId> (ty_vars, ... )? { <id> ( <binding> , ... ) , ... }
    Typedef {
        id: &'src str,
        ty_vars: Vec<&'src str>,
        arms: Vec<ParsedConstructorArm<'src>>,
    },

    // fn <id>( <binding> , ... ) -> <type> { <stmnt>; ... }
    Fn {
        name: &'src str,
        bindings: Vec<SpannedBinding<'src>>,
        ret_ty: ParsedType<'src>,
        body: Vec<SpannedStmt<'src, A>>,
    },
}

impl<'src> ParsedDefn<'src> {
    pub fn make_fn(
        offset: usize,
        id: &'src str,
        bindings: Vec<ParsedBinding<'src>>,
        ret: ParsedType<'src>,
        stmts: Vec<ParsedStmt<'src>>,
    ) -> Self {
        Spanned {
            offset,
            node: Defn::Fn {
                name: id,
                bindings,
                ret_ty: ret,
                body: stmts,
            },
        }
    }

    pub fn make_typedef(
        offset: usize,
        id: &'src str,
        ty_vars: Vec<&'src str>,
        arms: Vec<ParsedConstructorArm<'src>>,
    ) -> Self {
        Spanned {
            offset,
            node: Defn::Typedef { id, ty_vars, arms },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Stmt<'src, A> {
    Let(&'src str, SpannedExpr<'src, A>), // let <id> = <expr>;
    Read(ParsedType<'src>, &'src str),    // read <type> <id>;
    Echo(ParsedType<'src>, SpannedExpr<'src, A>), // echo <type> <expr>;
    Return(SpannedExpr<'src, A>),         // return <expr>;
}

impl<'src> ParsedStmt<'src> {
    pub fn make_let(offset: usize, id: &'src str, expr: ParsedExpr<'src>) -> Self {
        Spanned {
            offset,
            node: Stmt::Let(id, expr),
        }
    }

    pub fn make_read(offset: usize, typ: ParsedType<'src>, id: &'src str) -> Self {
        Spanned {
            offset,
            node: Stmt::Read(typ, id),
        }
    }

    pub fn make_echo(offset: usize, typ: ParsedType<'src>, expr: ParsedExpr<'src>) -> Self {
        Spanned {
            offset,
            node: Stmt::Echo(typ, expr),
        }
    }

    pub fn make_return(offset: usize, expr: ParsedExpr<'src>) -> Self {
        Spanned {
            offset,
            node: Stmt::Return(expr),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr<'src, A> {
    pub ann: A,
    pub kind: ExprKind<'src, A>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind<'src, A> {
    // atomic expressions
    Int(i64),
    Float(f64),
    Bool(bool),

    // idents
    Id(&'src str),

    // closures
    Fn(SpannedBinding<'src>, Box<SpannedExpr<'src, A>>), // fn ( <binding> ) -> <expr>

    // unary operators
    Neg(Box<SpannedExpr<'src, A>>),  // - <expr>
    FNeg(Box<SpannedExpr<'src, A>>), // -. <expr>
    Bang(Box<SpannedExpr<'src, A>>), // ! <expr>

    // function calls
    Call(Box<SpannedExpr<'src, A>>, Vec<SpannedExpr<'src, A>>), // <left>(<right>, ...)

    // binary operators
    Plus(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>), // <left> + <right>
    Minus(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>), // <left> - <right>
    Mult(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>), // <left> * <right>
    Div(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>),  // <left> / <right>
    FPlus(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>), // <left> +. <right>
    FMinus(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>), // <left> -. <right>
    FMult(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>), // <left> *. <right>
    FDiv(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>), // <left> /. <right>
    Pipe(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>), // <left> |> <right>
    Lt(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>),   // <left> < <right>
    LtEq(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>), // <left> <= <right>
    Gt(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>),   // <left> > <right>
    GtEq(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>), // <left> >= <right>
    FLt(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>),  // <left> <. <right>
    FLtEq(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>), // <left> <=. <right>
    FGt(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>),  // <left> >. <right>
    FGtEq(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>), // <left> >=. <right>
    Eq(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>),   // <left> == <right>
    NotEq(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>), // <left> != <right>
    Or(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>),   // <left> || <right>
    And(Box<SpannedExpr<'src, A>>, Box<SpannedExpr<'src, A>>),  // <left> && <right>

    // if <1> then <2> else <3>
    If(
        Box<SpannedExpr<'src, A>>,
        Box<SpannedExpr<'src, A>>,
        Box<SpannedExpr<'src, A>>,
    ),

    // match <expr> { <id>(<id>, ...) -> <expr>, ... }
    Match(Box<SpannedExpr<'src, A>>, Vec<SpannedMatchArm<'src, A>>),
}

// Display ignores the annotation entirely.
impl<'src, A> Display for Expr<'src, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl<'src, A> Display for ExprKind<'src, A>
where
    Expr<'src, A>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprKind::Int(num) => write!(f, "{num}"),
            ExprKind::Float(num) => write!(f, "{num}"),
            ExprKind::Bool(val) => write!(f, "{val}"),
            ExprKind::Id(id) => write!(f, "{id}"),
            ExprKind::Fn(binding, body) => write!(f, "(fn ({binding}) -> {body})"),
            ExprKind::Neg(expr) => write!(f, "-{expr}"),
            ExprKind::FNeg(expr) => write!(f, "-.{expr}"),
            ExprKind::Bang(expr) => write!(f, "!{expr}"),
            ExprKind::Call(func, args) => {
                let args_str = args
                    .iter()
                    .map(|arg| format!("{arg}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{func}({args_str})")
            }
            ExprKind::Plus(left, right) => write!(f, "({left} + {right})"),
            ExprKind::Minus(left, right) => write!(f, "({left} - {right})"),
            ExprKind::Mult(left, right) => write!(f, "({left} * {right})"),
            ExprKind::Div(left, right) => write!(f, "({left} / {right})"),
            ExprKind::FPlus(left, right) => write!(f, "({left} +. {right})"),
            ExprKind::FMinus(left, right) => write!(f, "({left} -. {right})"),
            ExprKind::FMult(left, right) => write!(f, "({left} *. {right})"),
            ExprKind::FDiv(left, right) => write!(f, "({left} /. {right})"),
            ExprKind::Pipe(left, right) => write!(f, "({left} |> {right})"),
            ExprKind::Lt(left, right) => write!(f, "({left} < {right})"),
            ExprKind::LtEq(left, right) => write!(f, "({left} <= {right})"),
            ExprKind::Gt(left, right) => write!(f, "({left} > {right})"),
            ExprKind::GtEq(left, right) => write!(f, "({left} >= {right})"),
            ExprKind::FLt(left, right) => write!(f, "({left} <. {right})"),
            ExprKind::FLtEq(left, right) => write!(f, "({left} <=. {right})"),
            ExprKind::FGt(left, right) => write!(f, "({left} >. {right})"),
            ExprKind::FGtEq(left, right) => write!(f, "({left} >=. {right})"),
            ExprKind::Eq(left, right) => write!(f, "({left} == {right})"),
            ExprKind::NotEq(left, right) => write!(f, "({left} != {right})"),
            ExprKind::Or(left, right) => write!(f, "({left} || {right})"),
            ExprKind::And(left, right) => write!(f, "({left} && {right})"),
            ExprKind::If(cond, then_branch, else_branch) => {
                write!(f, "if {cond} then {then_branch} else {else_branch}")
            }
            ExprKind::Match(expr, cases) => {
                let cases_str = cases
                    .iter()
                    .map(|case| format!("{case}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "match {expr} {{ {cases_str} }}")
            }
        }
    }
}

impl<'src> ParsedExpr<'src> {
    pub fn new(offset: usize, kind: ExprKind<'src, ()>) -> Self {
        Spanned {
            offset,
            node: Expr { ann: (), kind },
        }
    }
}

impl<'src, A> ExprKind<'src, A> {
    pub fn binop(
        op: TokenKind,
        left: SpannedExpr<'src, A>,
        right: SpannedExpr<'src, A>,
    ) -> ExprKind<'src, A> {
        let (left, right) = (Box::new(left), Box::new(right));
        match op {
            TokenKind::Plus => ExprKind::Plus(left, right),
            TokenKind::Minus => ExprKind::Minus(left, right),
            TokenKind::Times => ExprKind::Mult(left, right),
            TokenKind::Divide => ExprKind::Div(left, right),
            TokenKind::FPlus => ExprKind::FPlus(left, right),
            TokenKind::FMinus => ExprKind::FMinus(left, right),
            TokenKind::FTimes => ExprKind::FMult(left, right),
            TokenKind::FDivide => ExprKind::FDiv(left, right),
            TokenKind::Pipe => ExprKind::Pipe(left, right),
            TokenKind::LessThan => ExprKind::Lt(left, right),
            TokenKind::LessThanOrEq => ExprKind::LtEq(left, right),
            TokenKind::GreaterThan => ExprKind::Gt(left, right),
            TokenKind::GreaterThanOrEq => ExprKind::GtEq(left, right),
            TokenKind::FLessThan => ExprKind::FLt(left, right),
            TokenKind::FLessThanOrEq => ExprKind::FLtEq(left, right),
            TokenKind::FGreaterThan => ExprKind::FGt(left, right),
            TokenKind::FGreaterThanOrEq => ExprKind::FGtEq(left, right),
            TokenKind::Eq => ExprKind::Eq(left, right),
            TokenKind::NotEq => ExprKind::NotEq(left, right),
            TokenKind::Or => ExprKind::Or(left, right),
            TokenKind::And => ExprKind::And(left, right),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type<'src> {
    Int,
    Float,
    Bool,
    TypeId(&'src str),
    TypeParam(&'src str),
    TypeApp(&'src str, Vec<ParsedType<'src>>),
    Fn(Box<ParsedType<'src>>, Box<ParsedType<'src>>),
}

impl<'src> ParsedType<'src> {
    pub fn new(offset: usize, typ: Type<'src>) -> Self {
        Spanned { offset, node: typ }
    }
}

impl Display for Type<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Bool => write!(f, "bool"),
            Type::TypeId(id) => write!(f, "{id}"),
            Type::TypeParam(id) => write!(f, "{id}"),
            Type::TypeApp(id, args) => {
                let args_str = args
                    .iter()
                    .map(|arg| format!("{arg}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "{id}({args_str})")
            }
            Type::Fn(arg_type, ret_type) => write!(f, "({arg_type} -> {ret_type})"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Binding<'src> {
    pub id: &'src str,
    pub typ: ParsedType<'src>,
}

impl Display for Binding<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.typ)
    }
}

impl<'src> ParsedBinding<'src> {
    pub fn new(offset: usize, id: &'src str, typ: ParsedType<'src>) -> Self {
        Spanned {
            offset,
            node: Binding { id, typ },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchArm<'src, A> {
    pub id: &'src str,
    pub bindings: Vec<&'src str>,
    pub expr: SpannedExpr<'src, A>,
}

impl<A> Display for MatchArm<'_, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}) -> {}",
            self.id,
            self.bindings.join(", "),
            self.expr
        )
    }
}

impl<'src> ParsedMatchArm<'src> {
    pub fn new(
        offset: usize,
        id: &'src str,
        bindings: Vec<&'src str>,
        expr: ParsedExpr<'src>,
    ) -> Self {
        Spanned {
            offset,
            node: MatchArm { id, bindings, expr },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstructorArm<'src> {
    pub id: &'src str,
    pub bindings: Vec<ParsedBinding<'src>>,
}

impl<'src> ParsedConstructorArm<'src> {
    pub fn new(offset: usize, id: &'src str, bindings: Vec<ParsedBinding<'src>>) -> Self {
        Spanned {
            offset,
            node: ConstructorArm { id, bindings },
        }
    }
}
