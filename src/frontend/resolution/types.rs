use std::collections::HashMap;

pub struct TypeId(usize);
pub struct TypeParamId(usize);
pub struct ValueId(usize);

pub enum Identifier {
    TypeId(TypeId),
    TypeParamId(TypeParamId),
    ValueId(ValueId),
}

pub enum ResolvedDefn {
    Typedef {
        id: TypeId,
        params: Vec<TypeParamId>,
        arms: Vec<(ValueId, Vec<ResolvedBinding>)>,
    },

    Fn {
        id: ValueId,
        params: Vec<ResolvedBinding>,
        ret_ty: ResolvedType,
        body: Vec<ResolvedStmt>,
    },
}

pub enum ResolvedStmt {
    Let(ValueId, ResolvedExpr),
    Read(ValueId, ResolvedType),
    Echo(ResolvedType, ResolvedExpr),
    Return(ResolvedExpr),
}

pub enum ResolvedExpr {
    Int(i64),
    Bool(bool),
    Float(f64),

    Id(ValueId),

    Fn(ResolvedBinding, Box<ResolvedExpr>),

    Neg(Box<ResolvedExpr>),
    FNeg(Box<ResolvedExpr>),
    Bang(Box<ResolvedExpr>),

    Call(Box<ResolvedExpr>, Box<ResolvedExpr>),

    Plus(Box<ResolvedExpr>, Box<ResolvedExpr>),
    Minus(Box<ResolvedExpr>, Box<ResolvedExpr>),
    FPlus(Box<ResolvedExpr>, Box<ResolvedExpr>),
    FMinus(Box<ResolvedExpr>, Box<ResolvedExpr>),
    Mult(Box<ResolvedExpr>, Box<ResolvedExpr>),
    FMult(Box<ResolvedExpr>, Box<ResolvedExpr>),
    Div(Box<ResolvedExpr>, Box<ResolvedExpr>),
    FDiv(Box<ResolvedExpr>, Box<ResolvedExpr>),
    Pipe(Box<ResolvedExpr>, Box<ResolvedExpr>),
    Lt(Box<ResolvedExpr>, Box<ResolvedExpr>),
    Gt(Box<ResolvedExpr>, Box<ResolvedExpr>),
    LtEq(Box<ResolvedExpr>, Box<ResolvedExpr>),
    GtEq(Box<ResolvedExpr>, Box<ResolvedExpr>),
    FLt(Box<ResolvedExpr>, Box<ResolvedExpr>),
    FGt(Box<ResolvedExpr>, Box<ResolvedExpr>),
    FLtEq(Box<ResolvedExpr>, Box<ResolvedExpr>),
    FGtEq(Box<ResolvedExpr>, Box<ResolvedExpr>),
    Eq(Box<ResolvedExpr>, Box<ResolvedExpr>),
    NotEq(Box<ResolvedExpr>, Box<ResolvedExpr>),
    Or(Box<ResolvedExpr>, Box<ResolvedExpr>),
    And(Box<ResolvedExpr>, Box<ResolvedExpr>),

    If(Box<ResolvedExpr>, Box<ResolvedExpr>, Box<ResolvedExpr>),
    Match(Box<ResolvedExpr>, Vec<ResolvedMatchArm>),
}

pub enum ResolvedType {
    Int,
    Float,
    Bool,

    TypeId(TypeId),
    TypeParam(TypeParamId),
    TypeApp(TypeId, Vec<ResolvedType>),

    Fn(Box<ResolvedType>, Box<ResolvedType>),
}

pub struct ResolvedBinding {
    id: ValueId,
    typ: ResolvedType,
}

pub struct ResolvedMatchArm {
    id: ValueId,
    bindings: Vec<ValueId>,
    expr: ResolvedExpr,
}

pub struct ResolverCtx<'a> {
    num_typeids: usize,
    num_typeparamids: usize,
    num_valueids: usize,

    env: Vec<Env<'a>>,
}

pub struct Env<'a> {
    values: HashMap<&'a str, Identifier>,
}

impl<'a> ResolverCtx<'a> {
    pub fn new() -> Self {
        Self {
            num_typeids: 0,
            num_typeparamids: 0,
            num_valueids: 0,
            env: Vec::new(),
        }
    }

    pub fn next_typeid(&mut self) -> TypeId {
        let id = TypeId(self.num_typeids);
        self.num_typeids += 1;
        id
    }

    pub fn next_typeparamid(&mut self) -> TypeParamId {
        let id = TypeParamId(self.num_typeparamids);
        self.num_typeparamids += 1;
        id
    }

    pub fn next_valueid(&mut self) -> ValueId {
        let id = ValueId(self.num_valueids);
        self.num_valueids += 1;
        id
    }
}
