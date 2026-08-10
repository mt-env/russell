use std::collections::HashMap;

use crate::frontend::types::Spanned;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TypeId(usize);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TypeParamId(usize);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ValueId(usize);

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub enum Identifier {
    TypeId(TypeId),
    TypeParamId(TypeParamId),
    ValueId(ValueId),
}

#[derive(PartialEq, Debug)]
pub enum ResolvedDefnKind {
    Typedef {
        id: TypeId,
        params: Vec<TypeParamId>,
        arms: Vec<(ValueId, Vec<ResolvedBindingData>)>,
    },

    Fn {
        id: ValueId,
        params: Vec<ResolvedBindingData>,
        ret_ty: ResolvedType,
        body: Vec<ResolvedStmtKind>,
    },
}

pub type ResolvedDefn = Spanned<ResolvedDefnKind>;

impl ResolvedDefn {
    pub fn mk_typedef(
        offset: usize,
        id: TypeId,
        params: Vec<TypeParamId>,
        arms: Vec<(ValueId, Vec<ResolvedBindingData>)>,
    ) -> Self {
        Spanned {
            offset,
            node: ResolvedDefnKind::Typedef { id, params, arms },
        }
    }

    pub fn mk_fn(
        offset: usize,
        id: ValueId,
        params: Vec<ResolvedBindingData>,
        ret_ty: ResolvedType,
        body: Vec<ResolvedStmtKind>,
    ) -> Self {
        Spanned {
            offset,
            node: ResolvedDefnKind::Fn {
                id,
                params,
                ret_ty,
                body,
            },
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ResolvedStmtKind {
    Let(ValueId, ResolvedExprKind),
    Read(ValueId, ResolvedType),
    Echo(ResolvedType, ResolvedExprKind),
    Return(ResolvedExprKind),
}

pub type ResolvedStmt = Spanned<ResolvedStmtKind>;

impl ResolvedStmt {
    pub fn mk_let(offset: usize, id: ValueId, expr: ResolvedExprKind) -> Self {
        Spanned {
            offset,
            node: ResolvedStmtKind::Let(id, expr),
        }
    }

    pub fn mk_read(offset: usize, id: ValueId, typ: ResolvedType) -> Self {
        Spanned {
            offset,
            node: ResolvedStmtKind::Read(id, typ),
        }
    }

    pub fn mk_echo(offset: usize, typ: ResolvedType, expr: ResolvedExprKind) -> Self {
        Spanned {
            offset,
            node: ResolvedStmtKind::Echo(typ, expr),
        }
    }

    pub fn mk_return(offset: usize, expr: ResolvedExprKind) -> Self {
        Spanned {
            offset,
            node: ResolvedStmtKind::Return(expr),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ResolvedExprKind {
    Int(i64),
    Bool(bool),
    Float(f64),

    Id(ValueId),

    Fn(ResolvedBindingData, Box<ResolvedExprKind>),

    Neg(Box<ResolvedExprKind>),
    FNeg(Box<ResolvedExprKind>),
    Bang(Box<ResolvedExprKind>),

    Call(Box<ResolvedExprKind>, Vec<ResolvedExprKind>),

    Plus(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    Minus(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    FPlus(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    FMinus(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    Mult(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    FMult(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    Div(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    FDiv(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    Pipe(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    Lt(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    Gt(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    LtEq(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    GtEq(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    FLt(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    FGt(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    FLtEq(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    FGtEq(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    Eq(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    NotEq(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    Or(Box<ResolvedExprKind>, Box<ResolvedExprKind>),
    And(Box<ResolvedExprKind>, Box<ResolvedExprKind>),

    If(
        Box<ResolvedExprKind>,
        Box<ResolvedExprKind>,
        Box<ResolvedExprKind>,
    ),
    Match(Box<ResolvedExprKind>, Vec<ResolvedMatchArm>),
}

#[derive(PartialEq, Debug)]
pub enum ResolvedType {
    Int,
    Float,
    Bool,

    TypeId(TypeId),
    TypeParam(TypeParamId),
    TypeApp(TypeId, Vec<ResolvedType>),

    Fn(Box<ResolvedType>, Box<ResolvedType>),
}

#[derive(PartialEq, Debug)]
pub struct ResolvedBindingData {
    id: ValueId,
    typ: ResolvedType,
}

pub type ResolvedBinding = Spanned<ResolvedBindingData>;

impl ResolvedBinding {
    pub fn new(offset: usize, id: ValueId, typ: ResolvedType) -> Self {
        Spanned {
            offset,
            node: ResolvedBindingData { id, typ },
        }
    }
}

impl ResolvedBindingData {
    pub fn new(id: ValueId, typ: ResolvedType) -> Self {
        Self { id, typ }
    }
}

#[derive(PartialEq, Debug)]
pub struct ResolvedMatchArm {
    id: ValueId,
    bindings: Vec<ValueId>,
    expr: ResolvedExprKind,
}

impl ResolvedMatchArm {
    pub fn new(id: ValueId, bindings: Vec<ValueId>, expr: ResolvedExprKind) -> Self {
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
        Self {
            num_typeids: 0,
            num_typeparamids: 0,
            num_valueids: 0,
            ids: HashMap::new(),
            env: vec![Env::new()],
        }
    }

    pub(super) fn next_typeid(&mut self) -> TypeId {
        let id = TypeId(self.num_typeids);
        self.num_typeids += 1;
        id
    }

    pub(super) fn next_typeparamid(&mut self) -> TypeParamId {
        let id = TypeParamId(self.num_typeparamids);
        self.num_typeparamids += 1;
        id
    }

    pub(super) fn next_valueid(&mut self) -> ValueId {
        let id = ValueId(self.num_valueids);
        self.num_valueids += 1;
        id
    }

    pub(super) fn add_value(&mut self, name: &'a str) -> ValueId {
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

    pub(super) fn add_value_nodup(&mut self, name: &'a str) -> ValueId {
        match self.lookup_value(name) {
            Some(_) => panic!("Duplicate value name: {}", name),
            None => self.add_value(name),
        }
    }

    pub(super) fn add_type(&mut self, name: &'a str) -> TypeId {
        let id = self.next_typeid();
        self.ids.insert(Identifier::TypeId(id), name);
        if let Some(env) = self.env.last_mut() {
            env.values.insert(name, Identifier::TypeId(id));
        } else {
            panic!("No current scope to add type to");
        }
        id
    }

    pub(super) fn add_type_nodup(&mut self, name: &'a str) -> TypeId {
        match self.lookup_type(name) {
            Some(_) => panic!("Duplicate type name: {}", name),
            None => self.add_type(name),
        }
    }

    pub(super) fn add_typeparam(&mut self, name: &'a str) -> TypeParamId {
        let id = self.next_typeparamid();
        self.ids.insert(Identifier::TypeParamId(id), name);
        if let Some(env) = self.env.last_mut() {
            env.values.insert(name, Identifier::TypeParamId(id));
        } else {
            panic!("No current scope to add type parameter to");
        }
        id
    }

    pub(super) fn push_scope(&mut self) {
        self.env.push(Env::new());
    }

    pub(super) fn pop_scope(&mut self) {
        self.env.pop();
    }

    pub(super) fn lookup(&self, name: &str) -> Option<Identifier> {
        for env in self.env.iter().rev() {
            if let Some(id) = env.values.get(name) {
                return Some(*id);
            }
        }
        None
    }

    pub(super) fn lookup_value(&self, name: &str) -> Option<ValueId> {
        match self.lookup(name) {
            Some(Identifier::ValueId(id)) => Some(id),
            _ => None,
        }
    }

    pub(crate) fn lookup_type(&self, name: &str) -> Option<TypeId> {
        match self.lookup(name) {
            Some(Identifier::TypeId(id)) => Some(id),
            _ => None,
        }
    }

    pub(crate) fn lookup_typeparam(&self, name: &str) -> Option<TypeParamId> {
        match self.lookup(name) {
            Some(Identifier::TypeParamId(id)) => Some(id),
            _ => None,
        }
    }
}

pub struct Env<'a> {
    values: HashMap<&'a str, Identifier>,
}

impl<'a> Default for Env<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Env<'a> {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}
