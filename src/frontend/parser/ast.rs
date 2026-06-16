use std::fmt::Display;

use crate::frontend::{lexer::token::TokenKind, types::Spanned};

pub type SpannedExpr<'a, A> = Spanned<Expr<'a, A>>;
pub type SpannedStmt<'a, A> = Spanned<Stmt<'a, A>>;
pub type SpannedDefn<'a, A> = Spanned<Defn<'a, A>>;
pub type SpannedBinding<'a> = Spanned<Binding<'a>>;

pub type ParsedExpr<'a> = SpannedExpr<'a, ()>;
pub type ParsedStmt<'a> = SpannedStmt<'a, ()>;
pub type ParsedDefn<'a> = SpannedDefn<'a, ()>;
pub type ParsedBinding<'a> = SpannedBinding<'a>;

#[derive(Debug, PartialEq)]
pub enum Defn<'a, A> {
    // typedef <typeId> { <id> ( <binding> , ... ) , ... }
    Typedef(&'a str, Vec<(&'a str, Vec<SpannedBinding<'a>>)>),

    // fn <id>( <binding> , ... ) -> <type> { <stmnt>; ... }
    Fn(&'a str, Vec<SpannedBinding<'a>>, Type<'a>, Vec<SpannedStmt<'a, A>>),
}

impl<'a> ParsedDefn<'a> {
    pub fn make_fn(
        offset: usize,
        id: &'a str,
        bindings: Vec<ParsedBinding<'a>>,
        ret: Type<'a>,
        stmts: Vec<ParsedStmt<'a>>,
    ) -> Self {
        Spanned {
            offset,
            node: Defn::Fn(id, bindings, ret, stmts),
        }
    }

    pub fn make_typedef(offset: usize, id: &'a str, arms: Vec<(&'a str, Vec<Spanned<Binding<'a>>>)>) -> Self {
        Spanned {
            offset,
            node: Defn::Typedef(id, arms),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Stmt<'a, A> {
    Let(&'a str, SpannedExpr<'a, A>),   // let <id> = <expr>;
    Read(Type<'a>, &'a str),            // read <type> <id>;
    Echo(Type<'a>, SpannedExpr<'a, A>), // echo <type> <expr>;
    Return(SpannedExpr<'a, A>),         // return <expr>;
}

impl<'a> ParsedStmt<'a> {
    pub fn make_let(offset: usize, id: &'a str, expr: ParsedExpr<'a>) -> Self {
        Spanned {
            offset,
            node: Stmt::Let(id, expr),
        }
    }

    pub fn make_read(offset: usize, typ: Type<'a>, id: &'a str) -> Self {
        Spanned {
            offset,
            node: Stmt::Read(typ, id),
        }
    }

    pub fn make_echo(offset: usize, typ: Type<'a>, expr: ParsedExpr<'a>) -> Self {
        Spanned {
            offset,
            node: Stmt::Echo(typ, expr),
        }
    }

    pub fn make_return(offset: usize, expr: ParsedExpr<'a>) -> Self {
        Spanned {
            offset,
            node: Stmt::Return(expr),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr<'a, A> {
    pub ann: A,
    pub kind: ExprKind<'a, A>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind<'a, A> {
    // atomic expressions
    Int(i64),
    Float(f64),
    Bool(bool),

    // idents
    Id(&'a str),

    // closures
    Fn(SpannedBinding<'a>, Box<SpannedExpr<'a, A>>), // fn ( <binding> ) -> <expr>

    // unary operators
    Neg(Box<SpannedExpr<'a, A>>),  // - <expr>
    Bang(Box<SpannedExpr<'a, A>>), // ! <expr>

    // function calls
    Call(Box<SpannedExpr<'a, A>>, Vec<SpannedExpr<'a, A>>), // <left>(<right>, ...)

    // binary operators
    Plus(Box<SpannedExpr<'a, A>>, Box<SpannedExpr<'a, A>>), // <left> + <right>
    Minus(Box<SpannedExpr<'a, A>>, Box<SpannedExpr<'a, A>>), // <left> - <right>
    Mult(Box<SpannedExpr<'a, A>>, Box<SpannedExpr<'a, A>>), // <left> * <right>
    Div(Box<SpannedExpr<'a, A>>, Box<SpannedExpr<'a, A>>),  // <left> / <right>
    Pipe(Box<SpannedExpr<'a, A>>, Box<SpannedExpr<'a, A>>), // <left> |> <right>
    Less(Box<SpannedExpr<'a, A>>, Box<SpannedExpr<'a, A>>), // <left> < <right>
    LessEq(Box<SpannedExpr<'a, A>>, Box<SpannedExpr<'a, A>>), // <left> <= <right>
    Greater(Box<SpannedExpr<'a, A>>, Box<SpannedExpr<'a, A>>), // <left> > <right>
    GreaterEq(Box<SpannedExpr<'a, A>>, Box<SpannedExpr<'a, A>>), // <left> >= <right>
    Eq(Box<SpannedExpr<'a, A>>, Box<SpannedExpr<'a, A>>),   // <left> == <right>
    NotEq(Box<SpannedExpr<'a, A>>, Box<SpannedExpr<'a, A>>), // <left> != <right>
    Or(Box<SpannedExpr<'a, A>>, Box<SpannedExpr<'a, A>>),   // <left> || <right>
    And(Box<SpannedExpr<'a, A>>, Box<SpannedExpr<'a, A>>),  // <left> && <right>

    // if <1> then <2> else <3>
    If(
        Box<SpannedExpr<'a, A>>,
        Box<SpannedExpr<'a, A>>,
        Box<SpannedExpr<'a, A>>,
    ),

    // match <expr> { <id>(<bindings>) -> <expr>, ... }
    Match(
        Box<SpannedExpr<'a, A>>,
        Vec<(&'a str, Vec<SpannedBinding<'a>>, SpannedExpr<'a, A>)>,
    ),
}

// Display ignores the annotation entirely.
impl<'a, A> Display for Expr<'a, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl<'a, A> Display for ExprKind<'a, A>
where
    Expr<'a, A>: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExprKind::Int(num) => write!(f, "{num}"),
            ExprKind::Float(num) => write!(f, "{num}"),
            ExprKind::Bool(val) => write!(f, "{val}"),
            ExprKind::Id(id) => write!(f, "{id}"),
            ExprKind::Fn(binding, body) => write!(f, "(fn ({binding}) -> {body})"),
            ExprKind::Neg(expr) => write!(f, "-{expr}"),
            ExprKind::Bang(expr) => write!(f, "!{expr}"),
            ExprKind::Call(func, args) => {
                let args_str = args.iter().map(|arg| format!("{arg}")).collect::<Vec<_>>().join(", ");
                write!(f, "{func}({args_str})")
            }
            ExprKind::Plus(left, right) => write!(f, "({left} + {right})"),
            ExprKind::Minus(left, right) => write!(f, "({left} - {right})"),
            ExprKind::Mult(left, right) => write!(f, "({left} * {right})"),
            ExprKind::Div(left, right) => write!(f, "({left} / {right})"),
            ExprKind::Pipe(left, right) => write!(f, "({left} |> {right})"),
            ExprKind::Less(left, right) => write!(f, "({left} < {right})"),
            ExprKind::LessEq(left, right) => write!(f, "({left} <= {right})"),
            ExprKind::Greater(left, right) => write!(f, "({left} > {right})"),
            ExprKind::GreaterEq(left, right) => write!(f, "({left} >= {right})"),
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
                    .map(|(id, bindings, case_expr)| {
                        let bindings_str = bindings
                            .iter()
                            .map(|binding| format!("{binding}"))
                            .collect::<Vec<_>>()
                            .join(", ");
                        format!("{id}({bindings_str}) -> {case_expr}")
                    })
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "match {expr} {{ {cases_str} }}")
            }
        }
    }
}

impl<'a> ParsedExpr<'a> {
    pub fn new(offset: usize, kind: ExprKind<'a, ()>) -> Self {
        Spanned {
            offset,
            node: Expr { ann: (), kind },
        }
    }
}

impl<'a, A> ExprKind<'a, A> {
    pub fn binop(op: TokenKind, left: SpannedExpr<'a, A>, right: SpannedExpr<'a, A>) -> ExprKind<'a, A> {
        let (left, right) = (Box::new(left), Box::new(right));
        match op {
            TokenKind::Plus => ExprKind::Plus(left, right),
            TokenKind::Minus => ExprKind::Minus(left, right),
            TokenKind::Times => ExprKind::Mult(left, right),
            TokenKind::Divide => ExprKind::Div(left, right),
            TokenKind::Pipe => ExprKind::Pipe(left, right),
            TokenKind::LessThan => ExprKind::Less(left, right),
            TokenKind::LessThanOrEq => ExprKind::LessEq(left, right),
            TokenKind::GreaterThan => ExprKind::Greater(left, right),
            TokenKind::GreaterThanOrEq => ExprKind::GreaterEq(left, right),
            TokenKind::Eq => ExprKind::Eq(left, right),
            TokenKind::NotEq => ExprKind::NotEq(left, right),
            TokenKind::Or => ExprKind::Or(left, right),
            TokenKind::And => ExprKind::And(left, right),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type<'a> {
    Int,
    Float,
    Bool,
    TypeId(&'a str),
    Fn(Box<Type<'a>>, Box<Type<'a>>),
}

impl Display for Type<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Bool => write!(f, "bool"),
            Type::TypeId(id) => write!(f, "{id}"),
            Type::Fn(arg_type, ret_type) => write!(f, "({arg_type} -> {ret_type})"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Binding<'a> {
    pub id: &'a str,
    pub typ: Type<'a>,
}

impl Display for Binding<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.typ)
    }
}

impl<'a> ParsedBinding<'a> {
    pub fn new(offset: usize, id: &'a str, typ: Type<'a>) -> Self {
        Spanned {
            offset,
            node: Binding { id, typ },
        }
    }
}
