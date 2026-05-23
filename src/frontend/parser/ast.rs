use std::fmt::Display;

use crate::frontend::lexer::token::Token;

pub type ParsedExpr<'a> = Expr<'a, ()>;
pub type ParsedStmt<'a> = Stmt<'a, ()>;
pub type ParsedDefn<'a> = Defn<'a, ()>;

#[derive(Debug, PartialEq)]
pub enum Defn<'a, A> {
    // typedef <typeId> { <id> ( <binding> , ... ) , ... }
    Typedef(&'a str, Vec<(&'a str, Vec<Binding<'a>>)>),

    // fn <id>( <binding> , ... ) -> <type> { <stmnt>; ... }
    Fn(&'a str, Vec<Binding<'a>>, Type<'a>, Vec<Stmt<'a, A>>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt<'a, A> {
    Let(&'a str, Expr<'a, A>),  // let <id> = <expr>;
    Read(Type<'a>, &'a str),    // read <type> <id>;
    Echo(Type<'a>, Expr<'a, A>),   // echo <type> <expr>;
    Return(Expr<'a, A>),       // return <expr>;
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
    Fn(Binding<'a>, Box<Expr<'a, A>>), // fn ( <binding> ) -> <expr>

    // unary operators
    Neg(Box<Expr<'a, A>>),  // - <expr>
    Bang(Box<Expr<'a, A>>), // ! <expr>

    // function calls
    Call(Box<Expr<'a, A>>, Vec<Expr<'a, A>>), // <left>(<right>, ...)

    // binary operators
    Plus(Box<Expr<'a, A>>, Box<Expr<'a, A>>),      // <left> + <right>
    Minus(Box<Expr<'a, A>>, Box<Expr<'a, A>>),     // <left> - <right>
    Mult(Box<Expr<'a, A>>, Box<Expr<'a, A>>),      // <left> * <right>
    Div(Box<Expr<'a, A>>, Box<Expr<'a, A>>),       // <left> / <right>
    Pipe(Box<Expr<'a, A>>, Box<Expr<'a, A>>),      // <left> |> <right>
    Less(Box<Expr<'a, A>>, Box<Expr<'a, A>>),      // <left> < <right>
    LessEq(Box<Expr<'a, A>>, Box<Expr<'a, A>>),    // <left> <= <right>
    Greater(Box<Expr<'a, A>>, Box<Expr<'a, A>>),   // <left> > <right>
    GreaterEq(Box<Expr<'a, A>>, Box<Expr<'a, A>>), // <left> >= <right>
    Eq(Box<Expr<'a, A>>, Box<Expr<'a, A>>),        // <left> == <right>
    NotEq(Box<Expr<'a, A>>, Box<Expr<'a, A>>),     // <left> != <right>
    Or(Box<Expr<'a, A>>, Box<Expr<'a, A>>),        // <left> || <right>
    And(Box<Expr<'a, A>>, Box<Expr<'a, A>>),       // <left> && <right>

    // if <1> then <2> else <3>
    If(Box<Expr<'a, A>>, Box<Expr<'a, A>>, Box<Expr<'a, A>>),

    // match <expr> { <id>(<bindings>) -> <expr>, ... }
    Match(Box<Expr<'a, A>>, Vec<(&'a str, Vec<Binding<'a>>, Expr<'a, A>)>),
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

impl <'a> Expr<'a, ()> {
    pub fn parsed(kind: ExprKind<'a, ()>) -> Self {
        Expr { ann: (), kind }
    }
}

impl<'a, A> ExprKind<'a, A> {
    pub fn binop(op: Token, left: Expr<'a, A>, right: Expr<'a, A>) -> ExprKind<'a, A> {
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
            Type::Fn(arg_type, ret_type) => write!(
                f,
                "({arg_type} -> {ret_type})"
            ),
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

impl Binding<'_> {
    pub fn new<'a>(id: &'a str, typ: Type<'a>) -> Binding<'a> {
        Binding { id, typ }
    }
}
