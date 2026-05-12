use std::fmt::Display;

use crate::frontend::lexer::token::Token;

pub type ParsedExpr = Expr<()>;
pub type ParsedStmt = Stmt<()>;
pub type ParsedDefn = Defn<()>;

#[derive(Debug, PartialEq)]
pub enum Defn<A> {
    // typedef <typeId> { <id> ( <binding> , ... ) , ... };
    Typedef(String, Vec<(String, Vec<Binding>)>),

    // fn <id>( <binding> , ... ) -> <type> { <stmnt>; ... }
    Fn(String, Vec<Binding>, Type, Vec<Stmt<A>>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt<A> {
    Let(String, Expr<A>),  // let <id> = <expr>;
    Read(Type, String),    // read <type> <id>;
    Echo(Type, Expr<A>),   // echo <type> <expr>;
    Return(Expr<A>),       // return <expr>;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Expr<A> {
    pub ann: A,
    pub kind: ExprKind<A>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprKind<A> {
    // atomic expressions
    Int(i64),
    Float(f64),
    Bool(bool),

    // idents
    Id(String),

    // closures
    Fn(Binding, Box<Expr<A>>), // fn ( <binding> ) -> <expr>

    // unary operators
    Neg(Box<Expr<A>>),  // - <expr>
    Bang(Box<Expr<A>>), // ! <expr>

    // function calls
    Call(Box<Expr<A>>, Vec<Expr<A>>), // <left>(<right>, ...)

    // binary operators
    Plus(Box<Expr<A>>, Box<Expr<A>>),      // <left> + <right>
    Minus(Box<Expr<A>>, Box<Expr<A>>),     // <left> - <right>
    Mult(Box<Expr<A>>, Box<Expr<A>>),      // <left> * <right>
    Div(Box<Expr<A>>, Box<Expr<A>>),       // <left> / <right>
    Pipe(Box<Expr<A>>, Box<Expr<A>>),      // <left> |> <right>
    Less(Box<Expr<A>>, Box<Expr<A>>),      // <left> < <right>
    LessEq(Box<Expr<A>>, Box<Expr<A>>),    // <left> <= <right>
    Greater(Box<Expr<A>>, Box<Expr<A>>),   // <left> > <right>
    GreaterEq(Box<Expr<A>>, Box<Expr<A>>), // <left> >= <right>
    Eq(Box<Expr<A>>, Box<Expr<A>>),        // <left> == <right>
    NotEq(Box<Expr<A>>, Box<Expr<A>>),     // <left> != <right>
    Or(Box<Expr<A>>, Box<Expr<A>>),        // <left> || <right>
    And(Box<Expr<A>>, Box<Expr<A>>),       // <left> && <right>

    // if <1> then <2> else <3>
    If(Box<Expr<A>>, Box<Expr<A>>, Box<Expr<A>>),

    // match <expr> { <id>(<bindings>) -> <expr>, ... }
    Match(Box<Expr<A>>, Vec<(String, Vec<Binding>, Expr<A>)>),
}

// Display ignores the annotation entirely.
impl<A> Display for Expr<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl<A> Display for ExprKind<A>
where
    Expr<A>: Display,
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
                let args_str = args.iter().map(
                    |arg| format!("{arg}")
                ).collect::<Vec<_>>().join(", ");
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
            ExprKind::If(cond, then_branch, else_branch) => write!(
                f,
                "if {cond} then {then_branch} else {else_branch}"
            ),
            ExprKind::Match(expr, cases) => {
                let cases_str = cases.iter().map(|(id, bindings, case_expr)| {
                    let bindings_str = bindings.iter().map(
                        |binding| format!("{binding}")
                    ).collect::<Vec<_>>().join(", ");
                    format!("{id}({bindings_str}) -> {case_expr}")
                }).collect::<Vec<_>>().join(", ");
                write!(f, "match {expr} {{ {cases_str} }}")
            }
        }
    }
}

impl Expr<()> {
    pub fn parsed(kind: ExprKind<()>) -> Self {
        Expr { ann: (), kind }
    }
}

impl<A> ExprKind<A> {
    pub fn binop(op: Token, left: Expr<A>, right: Expr<A>) -> ExprKind<A> {
        let (left, right) = (Box::new(left), Box::new(right));
        match op {
            Token::Plus => ExprKind::Plus(left, right),
            Token::Minus => ExprKind::Minus(left, right),
            Token::Times => ExprKind::Mult(left, right),
            Token::Divide => ExprKind::Div(left, right),
            Token::Pipe => ExprKind::Pipe(left, right),
            Token::LessThan => ExprKind::Less(left, right),
            Token::LessThanOrEq => ExprKind::LessEq(left, right),
            Token::GreaterThan => ExprKind::Greater(left, right),
            Token::GreaterThanOrEq => ExprKind::GreaterEq(left, right),
            Token::Eq => ExprKind::Eq(left, right),
            Token::NotEq => ExprKind::NotEq(left, right),
            Token::Or => ExprKind::Or(left, right),
            Token::And => ExprKind::And(left, right),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    TypeId(String),
    Fn(Box<Type>, Box<Type>),
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Bool => write!(f, "bool"),
            Type::TypeId(id) => write!(f, "{id}"),
            Type::Fn(arg_type, ret_type) => write!(
                f,
                "({arg_type} -> {ret_type})"
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Binding {
    pub id: String,
    pub typ: Type,
}

impl Display for Binding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.id, self.typ)
    }
}

impl Binding {
    pub fn new(id: String, typ: Type) -> Binding {
        Binding { id, typ }
    }
}
