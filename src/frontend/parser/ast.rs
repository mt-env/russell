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
            Type::Fn(arg_type, ret_type) => write!(f, "({arg_type} -> {ret_type})"),
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
