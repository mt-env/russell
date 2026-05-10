use std::fmt::Display;

use crate::frontend::lexer::token::Token;

#[derive(Debug, PartialEq)]
pub enum Defn {
    // typedef <typeId> { <id> ( <binding> , ... ) , ... };
    Typedef(String, Vec<(String, Vec<Binding>)>),

    // fn <id>( <binding> , ... ) -> <type> { <stmnt>; ... }
    Fn(String, Vec<Binding>, Type, Vec<Stmt>),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Let(String, Expr),  // let <id> = <expr>;
    Read(Type, String), // read <type> <id>;
    Echo(Type, Expr),   // echo <type> <expr>;
    Return(Expr),       // return <expr>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    // atomic expressions
    Int(i64),
    Float(f64),
    Bool(bool),

    // idents
    Id(String),

    // closures
    Fn(Binding, Box<Expr>), // fn ( <binding> ) -> <expr>

    // unary operators
    Neg(Box<Expr>),  // - <exp>
    Bang(Box<Expr>), // ! <exp>

    // function calls
    Call(Box<Expr>, Vec<Expr>), // <left>(<right>, ...)

    // binary operators
    Plus(Box<Expr>, Box<Expr>),      // <left> + <right>
    Minus(Box<Expr>, Box<Expr>),     // <left> - <right>
    Mult(Box<Expr>, Box<Expr>),      // <left> * <right>
    Div(Box<Expr>, Box<Expr>),       // <left> / <right>
    Pipe(Box<Expr>, Box<Expr>),      // <left> |> <right>
    Less(Box<Expr>, Box<Expr>),      // <left> < <right>
    LessEq(Box<Expr>, Box<Expr>),    // <left> <= <right>
    Greater(Box<Expr>, Box<Expr>),   // <left> > <right>
    GreaterEq(Box<Expr>, Box<Expr>), // <left> >= <right>
    Eq(Box<Expr>, Box<Expr>),        // <left> == <right>
    NotEq(Box<Expr>, Box<Expr>),     // <left> != <right>
    Or(Box<Expr>, Box<Expr>),        // <left> || <right>
    And(Box<Expr>, Box<Expr>),       // <left> && <right>

    // if <1> then <2> else <3>
    If(Box<Expr>, Box<Expr>, Box<Expr>),

    // match <expr> { <id>(<bindings>) -> <expr>, ... }
    Match(Box<Expr>, Vec<(String, Vec<Binding>, Expr)>),
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Int(num) => write!(f, "{num}"),
            Expr::Float(num) => write!(f, "{num}"),
            Expr::Bool(val) => write!(f, "{val}"),
            Expr::Id(id) => write!(f, "{id}"),
            Expr::Fn(binding, body) => write!(f, "(fn ({binding}) -> {body})"),
            Expr::Neg(expr) => write!(f, "-{expr}"),
            Expr::Bang(expr) => write!(f, "!{expr}"),
            Expr::Call(func, args) => {
                let args_str = args.iter().map(
                    |arg| format!("{arg}")
                ).collect::<Vec<_>>().join(", ");
                write!(f, "{func}({args_str})")
            }
            Expr::Plus(left, right) => write!(f, "({left} + {right})"),
            Expr::Minus(left, right) => write!(f, "({left} - {right})"),
            Expr::Mult(left, right) => write!(f, "({left} * {right})"),
            Expr::Div(left, right) => write!(f, "({left} / {right})"),
            Expr::Pipe(left, right) => write!(f, "({left} |> {right})"),
            Expr::Less(left, right) => write!(f, "({left} < {right})"),
            Expr::LessEq(left, right) => write!(f, "({left} <= {right})"),
            Expr::Greater(left, right) => write!(f, "({left} > {right})"),
            Expr::GreaterEq(left, right) => write!(f, "({left} >= {right})"),
            Expr::Eq(left, right) => write!(f, "({left} == {right})"),
            Expr::NotEq(left, right) => write!(f, "({left} != {right})"),
            Expr::Or(left, right) => write!(f, "({left} || {right})"),
            Expr::And(left, right) => write!(f, "({left} && {right})"),
            Expr::If(cond, then_branch, else_branch) => write!(
                f,
                "if {cond} then {then_branch} else {else_branch}"
            ),
            Expr::Match(expr, cases) => {
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

impl Expr {
    pub fn binop(op: Token, left: Expr, right: Expr) -> Expr {
        let (left, right) = (Box::new(left), Box::new(right));
        match op {
            Token::Plus => Expr::Plus(left, right),
            Token::Minus => Expr::Minus(left, right),
            Token::Times => Expr::Mult(left, right),
            Token::Divide => Expr::Div(left, right),
            Token::Pipe => Expr::Pipe(left, right),
            Token::LessThan => Expr::Less(left, right),
            Token::LessThanOrEq => Expr::LessEq(left, right),
            Token::GreaterThan => Expr::Greater(left, right),
            Token::GreaterThanOrEq => Expr::GreaterEq(left, right),
            Token::Eq => Expr::Eq(left, right),
            Token::NotEq => Expr::NotEq(left, right),
            Token::Or => Expr::Or(left, right),
            Token::And => Expr::And(left, right),
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
