use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TypeId(usize);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct TypeParamId(usize);

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct ValueId(usize);

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
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

    Call(Box<ResolvedExpr>, Vec<ResolvedExpr>),

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

impl ResolvedMatchArm {
    pub fn new(id: ValueId, bindings: Vec<ValueId>, expr: ResolvedExpr) -> Self {
        Self { id, bindings, expr }
    }
}

pub struct ResolverCtx<'a> {
    num_typeids: usize,
    num_typeparamids: usize,
    num_valueids: usize,

    ids: HashMap<Identifier, &'a str>,

    env: Vec<Env<'a>>,
}

impl<'a> ResolverCtx<'a> {
    pub(crate) fn new() -> Self {
        let mut env = Vec::new();
        env.push(Env::new());
        Self {
            num_typeids: 0,
            num_typeparamids: 0,
            num_valueids: 0,
            ids: HashMap::new(),
            env,
        }
    }

    pub(crate) fn next_typeid(&mut self) -> TypeId {
        let id = TypeId(self.num_typeids);
        self.num_typeids += 1;
        id
    }

    pub(crate) fn next_typeparamid(&mut self) -> TypeParamId {
        let id = TypeParamId(self.num_typeparamids);
        self.num_typeparamids += 1;
        id
    }

    pub(crate) fn next_valueid(&mut self) -> ValueId {
        let id = ValueId(self.num_valueids);
        self.num_valueids += 1;
        id
    }

    pub(crate) fn add_value(&mut self, name: &'a str) -> ValueId {
        let id = self.next_valueid();
        self.ids.insert(Identifier::ValueId(id), name);
        // insert into the current scope so lookups find newly-declared names
        if let Some(env) = self.env.last_mut() {
            env.values.insert(name, Identifier::ValueId(id));
        } else {
            panic!("No current scope to add value to");
        }
        id
    }

    pub(crate) fn push_scope(&mut self) {
        self.env.push(Env::new());
    }

    pub(crate) fn pop_scope(&mut self) {
        self.env.pop();
    }

    pub(crate) fn lookup(&self, name: &str) -> Option<Identifier> {
        for env in self.env.iter().rev() {
            if let Some(id) = env.values.get(name) {
                return Some(*id);
            }
        }
        None
    }
}

pub struct Env<'a> {
    values: HashMap<&'a str, Identifier>,
}

impl<'a> Env<'a> {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}
